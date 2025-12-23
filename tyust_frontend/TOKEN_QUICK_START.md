# Token 验证功能 - 快速开始指南

> 5分钟快速了解和使用 Token 验证和自动登录功能

## 🎯 功能简介

当用户的 Token 失效时，系统会：
1. 自动检测并清除过期信息
2. 显示友好提示"登录已失效，请重新登录"
3. 自动跳转到登录页
4. 保留用户的账号密码（方便快速登录）

## 📁 核心文件（3个）

```
utils/
├── auth.js        # Token 管理工具（统一管理认证信息）
├── pageAuth.js    # 页面认证检查（自动检查登录状态）
└── request.js     # 网络请求封装（自动处理 Token 失效）✨已更新
```

## 🚀 5分钟快速使用

### 1️⃣ 需要登录的页面（推荐写法）

```javascript
// pages/score/index.js
const pageAuth = require('../../utils/pageAuth')

Page({
  onLoad(options) {
    // 一行代码搞定登录检查！
    if (!pageAuth.checkAuth()) return
    
    // 后面的代码只有登录后才会执行
    this.loadData()
  }
})
```

### 2️⃣ 登录页面

```javascript
// pages/login/index.js
const auth = require('../../utils/auth')

Page({
  login() {
    loginRequest({ stuId, password }).then(res => {
      // 保存 Token 和用户信息
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

### 3️⃣ 退出登录

```javascript
// pages/mine/index.js
const auth = require('../../utils/auth')

Page({
  logout() {
    wx.showModal({
      content: '确定要退出登录吗？',
      success: (res) => {
        if (res.confirm) {
          // 清除认证信息（保留账号密码）
          auth.clearAuth(true)
          
          // 跳转登录页
          wx.reLaunch({ url: '/pages/login/index' })
        }
      }
    })
  }
})
```

### 4️⃣ 检查登录状态

```javascript
// 任意页面
const auth = require('../../utils/auth')

Page({
  onLoad() {
    // 方式1：只检查，不跳转
    if (auth.hasToken()) {
      console.log('已登录')
    }
    
    // 方式2：检查并自动跳转（推荐）
    if (!auth.checkLogin()) return
  }
})
```

## ✅ 已更新的页面

以下页面已经添加了登录检查，可以直接参考：

- ✅ `pages/login/index.js` - 登录页
- ✅ `pages/mine/index.js` - 个人中心
- ✅ `pages/score/index.js` - 成绩页
- ✅ `pages/course/index.js` - 课程页

## 🧪 快速测试

### 测试 Token 失效（3步）

```javascript
// 1. 在控制台设置无效 Token
wx.setStorageSync('token', 'invalid_token')

// 2. 刷新任意需要登录的页面（如成绩页）

// 3. 预期结果：
// ✅ 显示"登录已失效，请重新登录"
// ✅ 2秒后自动跳转到登录页
// ✅ 账号密码保留（如果之前勾选了记住）
```

### 测试未登录访问（3步）

```javascript
// 1. 在控制台清除 Token
wx.removeStorageSync('token')

// 2. 访问成绩页或课程页

// 3. 预期结果：
// ✅ 显示"请先登录"
// ✅ 自动跳转到登录页
```

## 🔑 常用 API

### auth 工具（utils/auth.js）

```javascript
const auth = require('../../utils/auth')

// 检查是否登录
auth.hasToken()                    // 返回 true/false

// 获取 Token
auth.getToken()                    // 返回 token 字符串

// 设置 Token
auth.setToken('your-token')        

// 获取用户信息
auth.getUserInfo()                 // 返回 { studentId, name, class, avatarUrl }

// 设置用户信息
auth.setUserInfo({ ... })          

// 清除认证信息
auth.clearAuth(true)               // true=保留账号密码，false=完全清除

// 检查登录（未登录会自动跳转）
auth.checkLogin()                  // 返回 true/false
```

### pageAuth 工具（utils/pageAuth.js）

```javascript
const pageAuth = require('../../utils/pageAuth')

// 检查页面认证（未登录自动跳转）
pageAuth.checkAuth()               // 返回 true/false

// 装饰器方式（高级用法）
Page(pageAuth.withAuth({
  onLoad() {
    // 只有登录后才会执行
  }
}, { checkOnLoad: true }))
```

## ⚠️ 注意事项

### ✅ 推荐做法

```javascript
// ✅ 使用 pageAuth.checkAuth() 检查登录
if (!pageAuth.checkAuth()) return

// ✅ 使用 auth.clearAuth(true) 保留账号
auth.clearAuth(true)

// ✅ 使用 wx.reLaunch 跳转登录页
wx.reLaunch({ url: '/pages/login/index' })
```

### ❌ 避免做法

```javascript
// ❌ 不要自己判断 token 是否为空
if (!wx.getStorageSync('token')) { ... }

// ❌ 不要手动清除存储
wx.clearStorageSync()

// ❌ 不要使用 wx.redirectTo 跳转登录页
wx.redirectTo({ url: '/pages/login/index' })
```

## 🔧 后端配置

确保后端返回正确的状态码：

```json
// ✅ 成功
{ "code": 0, "data": {...}, "msg": "success" }

// ✅ Token失效（会触发自动登录）
{ "code": 401, "msg": "登录已失效" }
{ "code": 403, "msg": "无权限" }

// ✅ 业务错误（不触发自动登录）
{ "code": -1, "msg": "错误信息" }
```

## 📚 详细文档

想了解更多？查看以下文档：

| 文档 | 说明 | 适合人群 |
|------|------|---------|
| [TOKEN_AUTH_README.md](./TOKEN_AUTH_README.md) | 功能概览 | 所有人 ⭐ |
| [TOKEN_AUTH_GUIDE.md](./TOKEN_AUTH_GUIDE.md) | 详细使用指南 | 开发者 |
| [TOKEN_TEST_GUIDE.md](./TOKEN_TEST_GUIDE.md) | 测试指南 | 测试人员 |
| [TOKEN_IMPLEMENTATION_SUMMARY.md](./TOKEN_IMPLEMENTATION_SUMMARY.md) | 实现总结 | 技术负责人 |
| [CHECKLIST.md](./CHECKLIST.md) | 检查清单 | 上线前必看 ✅ |

## ❓ 常见问题

### Q1: Token 失效后没有自动跳转？
**A:** 检查后端返回的 code 是否为 401 或 403

### Q2: 如何完全清除用户数据（包括账号）？
**A:** 使用 `auth.clearAuth(false)`

### Q3: 如何在 tabBar 页面使用？
**A:** tabBar 页面使用 `auth.hasToken()` 判断状态，不要强制跳转

### Q4: 多个请求同时失效会重复跳转吗？
**A:** 不会，已经做了防重复处理

## 🎉 开始使用

1. ✅ 所有核心文件已就绪
2. ✅ 参考已更新的页面
3. ✅ 使用上面的代码示例
4. ✅ 遇到问题查看详细文档

**现在就可以开始使用了！** 🚀

---

💡 **提示**: 建议先阅读 [TOKEN_AUTH_README.md](./TOKEN_AUTH_README.md) 了解完整功能