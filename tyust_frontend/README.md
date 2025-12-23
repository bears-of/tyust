# 云小慧

大学校园课表小程序实战课程开源项目

UI设计稿地址：[https://mastergo.com/goto/rpCIDmXp](https://mastergo.com/goto/rpCIDmXp)

后端项目地址：[https://github.com/danbaixi/BaiyunAPI](https://github.com/danbaixi/BaiyunAPI)

在线观看课程：[https://www.bilibili.com/video/BV1go4y1M7Fa](https://www.bilibili.com/video/BV1go4y1M7Fa)

> 请注意，虽然是白云学院的教务系统 API，我们也提供了测试号的功能，不需要白云教务系统账号也可以使用模拟数据！
> **测试号为 stuId: test，password: 123456**

## 课程配套文章
- [1.项目环境搭建](/articles/1.项目环境搭建.md)
- [2.小程序登录功能开发](/articles/2.小程序登录功能开发.md)
- [3.封装请求函数](/articles/3.封装请求函数.md)
- [4.环境变量配置.md](/articles/4.环境变量配置.md)

## ✨ 新功能：Token 验证和自动登录

已实现完整的 Token 验证和自动重新登录功能，提供更好的用户体验：

- ✅ Token 失效自动检测
- ✅ 自动清除过期认证信息
- ✅ 友好的提示信息
- ✅ 自动跳转到登录页
- ✅ 保留用户账号密码
- ✅ 防止重复跳转

### 📚 相关文档

| 文档 | 说明 | 推荐阅读 |
|------|------|---------|
| [TOKEN_QUICK_START.md](./TOKEN_QUICK_START.md) | 5分钟快速开始 | ⭐⭐⭐⭐⭐ |
| [TOKEN_AUTH_README.md](./TOKEN_AUTH_README.md) | 功能概览说明 | ⭐⭐⭐⭐⭐ |
| [TOKEN_AUTH_GUIDE.md](./TOKEN_AUTH_GUIDE.md) | 详细使用指南 | ⭐⭐⭐⭐ |
| [TOKEN_TEST_GUIDE.md](./TOKEN_TEST_GUIDE.md) | 测试指南 | ⭐⭐⭐ |
| [TOKEN_IMPLEMENTATION_SUMMARY.md](./TOKEN_IMPLEMENTATION_SUMMARY.md) | 实现总结 | ⭐⭐⭐ |
| [CHECKLIST.md](./CHECKLIST.md) | 上线检查清单 | ⭐⭐⭐⭐⭐ |

### 🚀 快速使用

```javascript
// 需要登录的页面
const pageAuth = require('../../utils/pageAuth')

Page({
  onLoad(options) {
    if (!pageAuth.checkAuth()) return
    this.loadData()
  }
})
```

更多使用方法请查看 [TOKEN_QUICK_START.md](./TOKEN_QUICK_START.md)

---

课程未完结，还在持续更新中...
