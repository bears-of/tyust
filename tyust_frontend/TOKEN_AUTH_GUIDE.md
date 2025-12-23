# Token 验证和自动登录功能使用指南

本文档介绍前端 Token 验证和自动重新登录功能的实现和使用方法。

## 功能概述

当用户的 Token 失效时，系统会自动：
1. 清除本地存储的认证信息
2. 显示友好的提示信息
3. 自动跳转到登录页面
4. 保留用户的账号密码（如果用户之前选择了记住密码）

## 核心文件

### 1. `utils/auth.js` - Token 管理工具

提供统一的 Token 和用户信息管理功能。

#### 主要方法

```javascript
const auth = require('../../utils/auth')

// 检查是否有 token
auth.hasToken() // 返回 boolean

// 获取 token
auth.getToken() // 返回 string

// 设置 token
auth.setToken('your-token-here')

// 移除 token
auth.removeToken()

// 获取学生ID
auth.getStudentId()

// 获取用户信息
const userInfo = auth.getUserInfo()
// 返回: { studentId, name, class, avatarUrl }

// 设置用户信息
auth.setUserInfo({
  studentId: '2021001',
  name: '张三',
  class: '计算机21-1',
  avatarUrl: 'https://...'
})

// 清除所有认证信息
auth.clearAuth(true) // 参数为true时保留账号密码

// 验证 token 是否有效
auth.validateToken().then(isValid => {
  console.log('Token有效:', isValid)
})

// 检查登录状态，未登录则跳转
if (auth.checkLogin()) {
  // 已登录，继续执行业务逻辑
}
```

### 2. `utils/request.js` - 网络请求封装

自动处理 Token 失效的情况。

#### Token 失效处理

当服务器返回 `code: 401` 或 `code: 403` 时，会自动：
1. 清除本地认证信息
2. 显示提示消息
3. 跳转到登录页面
4. 避免重复跳转（使用标志位控制）

#### 使用示例

```javascript
import createRequest from '../utils/request'

// 发起需要认证的请求
createRequest({
  url: '/scores',
  method: 'GET',
  data: { semester: '2023-2024-1' }
}).then(res => {
  console.log('成功:', res.data)
}).catch(err => {
  console.error('失败:', err)
})

// 发起不需要认证的请求
createRequest({
  url: '/auth/login',
  method: 'POST',
  data: { stuId, password },
  needLogin: false
}).then(res => {
  console.log('登录成功')
})
```

### 3. `utils/pageAuth.js` - 页面认证检查

提供页面级别的登录状态检查。

#### 使用方法

##### 方法一：手动调用检查

```javascript
const pageAuth = require('../../utils/pageAuth')

Page({
  onLoad(options) {
    // 检查登录状态，未登录会自动跳转
    if (!pageAuth.checkAuth()) {
      return
    }
    
    // 已登录，继续加载页面数据
    this.loadData()
  }
})
```

##### 方法二：使用装饰器（推荐）

```javascript
const pageAuth = require('../../utils/pageAuth')

// 使用 withAuth 包装页面配置
Page(pageAuth.withAuth({
  data: {
    list: []
  },
  
  onLoad(options) {
    // 如果未登录，不会执行到这里
    console.log('用户已登录')
    this.loadData()
  },
  
  loadData() {
    // 加载数据
  }
}, {
  checkOnLoad: true,  // 在 onLoad 时检查
  checkOnShow: false  // 在 onShow 时是否检查
}))
```

## 完整使用示例

### 示例1：需要登录的页面

```javascript
// pages/score/index.js
const auth = require('../../utils/auth')
const pageAuth = require('../../utils/pageAuth')
import { getScoreListRequest } from '../../api/main'

Page(pageAuth.withAuth({
  data: {
    scores: []
  },
  
  onLoad(options) {
    // 此时已确保用户已登录
    this.loadScores()
  },
  
  loadScores() {
    getScoreListRequest()
      .then(res => {
        if (res.code === 0) {
          this.setData({
            scores: res.data
          })
        }
      })
      .catch(err => {
        // token失效会自动跳转到登录页
        console.error('加载成绩失败:', err)
      })
  }
}, {
  checkOnLoad: true
}))
```

### 示例2：可选登录的页面（如个人中心）

```javascript
// pages/mine/index.js
const auth = require('../../utils/auth')

Page({
  data: {
    isLogin: false,
    userInfo: {}
  },
  
  onLoad(options) {
    // 不强制登录，只检查状态
    this.checkLoginStatus()
  },
  
  onShow() {
    this.checkLoginStatus()
  },
  
  checkLoginStatus() {
    const isLogin = auth.hasToken()
    
    if (isLogin) {
      const userInfo = auth.getUserInfo()
      this.setData({
        isLogin: true,
        userInfo
      })
    } else {
      this.setData({
        isLogin: false,
        userInfo: {}
      })
    }
  },
  
  // 需要登录的操作
  doSomethingNeedLogin() {
    if (!auth.checkLogin()) {
      return // 会自动跳转到登录页
    }
    
    // 执行需要登录的操作
  },
  
  // 退出登录
  logout() {
    wx.showModal({
      title: '提示',
      content: '确定要退出登录吗？',
      success: (res) => {
        if (res.confirm) {
          auth.clearAuth(true) // 保留账号密码
          
          this.setData({
            isLogin: false,
            userInfo: {}
          })
          
          wx.showToast({
            title: '已退出登录',
            icon: 'success'
          })
          
          setTimeout(() => {
            wx.reLaunch({
              url: '/pages/login/index'
            })
          }, 1500)
        }
      }
    })
  }
})
```

### 示例3：登录页面

```javascript
// pages/login/index.js
const auth = require('../../utils/auth')
import { loginRequest } from '../../api/main'

Page({
  data: {
    stuId: '',
    password: '',
    saveAccount: true
  },
  
  onLoad(options) {
    // 如果已经登录，直接跳转到首页
    if (auth.hasToken()) {
      wx.switchTab({
        url: '/pages/index/index'
      })
      return
    }
    
    // 加载保存的账号
    this.loadSavedAccount()
  },
  
  loadSavedAccount() {
    const account = wx.getStorageSync(auth.STORAGE_KEYS.ACCOUNT)
    if (account) {
      this.setData({
        stuId: account.stuId || '',
        password: account.password || '',
        saveAccount: true
      })
    }
  },
  
  login() {
    const { stuId, password, saveAccount } = this.data
    
    if (!stuId || !password) {
      wx.showToast({
        title: '请输入账号和密码',
        icon: 'none'
      })
      return
    }
    
    wx.showLoading({ title: '登录中...' })
    
    loginRequest({ stuId, password })
      .then(res => {
        wx.hideLoading()
        
        if (res.code === 0) {
          // 保存 token 和用户信息
          auth.setToken(res.data.token)
          auth.setUserInfo({
            studentId: res.data.studentId,
            name: res.data.name,
            class: res.data.class,
            avatarUrl: res.data.avatarUrl
          })
          
          // 保存账号（如果用户选择记住）
          if (saveAccount) {
            wx.setStorageSync(auth.STORAGE_KEYS.ACCOUNT, { stuId, password })
          } else {
            wx.removeStorageSync(auth.STORAGE_KEYS.ACCOUNT)
          }
          
          wx.showToast({
            title: '登录成功',
            icon: 'success'
          })
          
          setTimeout(() => {
            wx.switchTab({
              url: '/pages/index/index'
            })
          }, 1500)
        }
      })
      .catch(err => {
        wx.hideLoading()
        wx.showToast({
          title: err.msg || '登录失败',
          icon: 'none'
        })
      })
  }
})
```

## 服务器端配置

确保后端返回正确的状态码：

- **成功**: `{ code: 0, data: {...}, msg: 'success' }`
- **Token失效**: `{ code: 401, msg: '登录已失效' }` 或 `{ code: 403, msg: '无权限' }`
- **业务错误**: `{ code: -1, msg: '错误信息' }`

## 注意事项

1. **避免重复跳转**: `request.js` 中使用 `isHandlingTokenExpired` 标志位，防止多个请求同时失效时重复跳转

2. **保留用户数据**: 默认情况下，退出登录会保留账号密码和头像，方便用户下次登录

3. **清除所有数据**: 如果需要完全清除用户数据，使用 `auth.clearAuth(false)`

4. **页面生命周期**: 
   - 在需要登录的页面使用 `pageAuth.withAuth` 装饰器
   - 在可选登录的页面手动调用 `auth.checkLogin()` 或 `auth.hasToken()`

5. **Token验证**: 如果需要验证 token 是否真实有效，调用 `auth.validateToken()`（需要后端提供验证接口）

## 常见问题

### Q1: Token 失效后跳转多次？
**A**: 使用 `wx.reLaunch` 替代 `wx.redirectTo`，并确保使用了防重复标志位

### Q2: 如何在 tabBar 页面检查登录？
**A**: 在 `onShow` 中检查登录状态，但不要使用 `wx.redirectTo`，而是显示提示并引导用户点击登录

### Q3: 退出登录后仍然能访问数据？
**A**: 确保调用 `auth.clearAuth()` 清除了所有缓存数据

### Q4: 如何实现自动刷新 Token？
**A**: 在 `request.js` 中添加 Token 刷新逻辑，当 Token 快过期时自动调用刷新接口

## 更新日志

- **v1.0.0** (2024-01-XX): 初始版本，实现基础的 Token 验证和自动登录功能