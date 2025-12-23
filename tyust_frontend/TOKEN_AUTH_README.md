# Token 验证和自动登录功能说明

## 📋 功能概述

前端已实现完整的 Token 验证和自动重新登录功能：

- ✅ Token 失效自动检测
- ✅ 自动清除过期认证信息
- ✅ 友好的提示信息
- ✅ 自动跳转到登录页
- ✅ 保留用户账号密码（可选）
- ✅ 防止重复跳转

## 📁 核心文件

### 1. `utils/auth.js` - Token 管理工具
统一管理 Token 和用户信息的存储、读取、清除等操作。

**主要方法：**
```javascript
const auth = require('../../utils/auth')

auth.hasToken()           // 检查是否有token
auth.getToken()           // 获取token
auth.setToken(token)      // 设置token
auth.getUserInfo()        // 获取用户信息
auth.setUserInfo({...})   // 设置用户信息
auth.clearAuth(true)      // 清除认证信息（保留账号）
auth.checkLogin()         // 检查登录，未登录则跳转
```

### 2. `utils/request.js` - 网络请求封装
自动处理 Token 失效（code: 401 或 403），执行清除和跳转操作。

**特性：**
- 自动在请求头添加 token
- 检测 401/403 状态码
- 自动清除过期数据
- 防止重复跳转登录页

### 3. `utils/pageAuth.js` - 页面认证检查
提供页面级别的登录状态检查工具。

**使用方式：**
```javascript
const pageAuth = require('../../utils/pageAuth')

// 方式1：手动检查
Page({
  onLoad() {
    if (!pageAuth.checkAuth()) return
    // 已登录，继续执行
  }
})

// 方式2：装饰器（推荐）
Page(pageAuth.withAuth({
  onLoad() {
    // 只有登录后才会执行
  }
}, { checkOnLoad: true }))
```

## 🚀 使用示例

### 需要登录的页面（推荐写法）

```javascript
// pages/score/index.js
const auth = require('../../utils/auth')
const pageAuth = require('../../utils/pageAuth')
import { getScoreListRequest } from '../../api/main'

Page({
  data: { scores: [] },
  
  onLoad(options) {
    // 检查登录状态，未登录自动跳转
    if (!pageAuth.checkAuth()) return
    this.loadData()
  },
  
  loadData() {
    // Token失效时会自动处理
    getScoreListRequest().then(res => {
      this.setData({ scores: res.data })
    })
  }
})
```

### 登录页面

```javascript
// pages/login/index.js
const auth = require('../../utils/auth')
import { loginRequest } from '../../api/main'

Page({
  onLoad() {
    // 已登录则跳转首页
    if (auth.hasToken()) {
      wx.switchTab({ url: '/pages/index/index' })
    }
  },
  
  login() {
    loginRequest({ stuId, password }).then(res => {
      // 保存token和用户信息
      auth.setToken(res.data.token)
      auth.setUserInfo({
        studentId: res.data.studentId,
        name: res.data.name,
        class: res.data.class
      })
      
      // 跳转首页
      wx.switchTab({ url: '/pages/index/index' })
    })
  }
})
```

### 退出登录

```javascript
// pages/mine/index.js
const auth = require('../../utils/auth')

Page({
  logout() {
    wx.showModal({
      title: '提示',
      content: '确定要退出登录吗？',
      success: (res) => {
        if (res.confirm) {
          // 清除认证信息（保留账号密码）
          auth.clearAuth(true)
          
          wx.showToast({ title: '已退出登录', icon: 'success' })
          
          setTimeout(() => {
            wx.reLaunch({ url: '/pages/login/index' })
          }, 1500)
        }
      }
    })
  }
})
```

## 🔧 后端配置要求

确保后端返回正确的状态码：

```json
// 成功
{ "code": 0, "data": {...}, "msg": "success" }

// Token失效（会触发自动登录）
{ "code": 401, "msg": "登录已失效" }
{ "code": 403, "msg": "无权限访问" }

// 业务错误
{ "code": -1, "msg": "错误信息" }
```

## 📝 已更新的页面

以下页面已添加登录状态检查：

- ✅ `pages/login/index.js` - 登录页（检查是否已登录）
- ✅ `pages/mine/index.js` - 个人中心（展示登录状态）
- ✅ `pages/score/index.js` - 成绩查询（需要登录）
- ✅ `pages/course/index.js` - 课程表（需要登录）

## ⚠️ 注意事项

1. **防止重复跳转**：使用 `isHandlingTokenExpired` 标志位，避免多个请求同时失效时重复跳转

2. **保留用户数据**：默认退出登录会保留账号密码和头像，方便下次登录
   - 保留：`auth.clearAuth(true)`
   - 完全清除：`auth.clearAuth(false)`

3. **页面类型选择**：
   - 必须登录的页面：使用 `pageAuth.checkAuth()` 或 `pageAuth.withAuth()`
   - 可选登录的页面：使用 `auth.hasToken()` 判断状态

4. **跳转方式**：使用 `wx.reLaunch` 而不是 `wx.redirectTo`，避免返回栈问题

## 📚 完整文档

详细使用方法请参考：[TOKEN_AUTH_GUIDE.md](./TOKEN_AUTH_GUIDE.md)

## 🎯 测试场景

建议测试以下场景：

1. ✅ Token 正常使用
2. ✅ Token 过期后自动跳转登录
3. ✅ 未登录访问需要登录的页面
4. ✅ 已登录访问登录页自动跳转首页
5. ✅ 退出登录后清除数据
6. ✅ 多个请求同时失效不重复跳转
7. ✅ 重新登录后账号密码自动填充（如果之前勾选记住）

## 🔗 相关文件

- `utils/auth.js` - Token 管理工具
- `utils/request.js` - 网络请求封装
- `utils/pageAuth.js` - 页面认证检查
- `TOKEN_AUTH_GUIDE.md` - 详细使用指南