# Token 验证功能 - 文件目录结构

本文档说明 Token 验证和自动登录功能相关的所有文件及其作用。

## 📁 文件目录树

```
tyust_frontend/
│
├── utils/                                  # 工具类目录
│   ├── auth.js                            # ✨ Token 管理工具（新增）
│   ├── pageAuth.js                        # ✨ 页面认证检查工具（新增）
│   ├── request.js                         # ✨ 网络请求封装（已更新）
│   ├── util.js                            # 其他工具函数
│   └── tools.wxs                          # WXS 工具
│
├── pages/                                  # 页面目录
│   ├── login/
│   │   └── index.js                       # ✨ 登录页（已更新）
│   ├── mine/
│   │   └── index.js                       # ✨ 个人中心（已更新）
│   ├── score/
│   │   └── index.js                       # ✨ 成绩页（已更新）
│   ├── course/
│   │   └── index.js                       # ✨ 课程页（已更新）
│   └── ...                                # 其他页面
│
├── api/
│   └── main.js                            # API 接口定义
│
├── TOKEN_QUICK_START.md                   # ✨ 快速开始指南（新增）⭐⭐⭐⭐⭐
├── TOKEN_AUTH_README.md                   # ✨ 功能说明文档（新增）⭐⭐⭐⭐⭐
├── TOKEN_AUTH_GUIDE.md                    # ✨ 详细使用指南（新增）⭐⭐⭐⭐
├── TOKEN_TEST_GUIDE.md                    # ✨ 测试指南文档（新增）⭐⭐⭐
├── TOKEN_IMPLEMENTATION_SUMMARY.md        # ✨ 实现总结文档（新增）⭐⭐⭐
├── TOKEN_FILES_STRUCTURE.md               # ✨ 本文档（新增）
├── CHECKLIST.md                           # ✨ 检查清单（新增）⭐⭐⭐⭐⭐
├── README.md                              # ✨ 项目 README（已更新）
│
└── ...                                    # 其他文件
```

## 📝 文件详细说明

### 核心文件（必读）

#### 1. `utils/auth.js` (187 行)
**功能：** Token 和用户认证管理工具类

**核心功能：**
- Token 的存储、获取、删除
- 用户信息的管理
- 登录状态检查
- 认证信息清除
- Token 验证

**何时使用：**
- 需要检查登录状态时
- 需要保存/读取用户信息时
- 退出登录时

**引入方式：**
```javascript
const auth = require('../../utils/auth')
```

---

#### 2. `utils/pageAuth.js` (121 行)
**功能：** 页面级别的认证检查工具

**核心功能：**
- 页面加载时自动检查登录
- 装饰器模式包装页面
- 统一的认证跳转逻辑

**何时使用：**
- 页面需要强制登录时
- 想要减少重复代码时

**引入方式：**
```javascript
const pageAuth = require('../../utils/pageAuth')
```

---

#### 3. `utils/request.js` (124 行，已更新)
**功能：** 网络请求封装，自动处理 Token 失效

**核心改动：**
- 引入 auth 工具管理 Token
- 自动检测 401/403 状态码
- 统一的 Token 失效处理
- 防止重复跳转登录页

**何时使用：**
- 所有 API 请求都通过此文件
- 自动在请求头添加 Token
- 自动处理 Token 失效

**已被以下文件使用：**
- `api/main.js` 中的所有接口

---

### 页面文件（已更新）

#### 4. `pages/login/index.js` (221 行，已更新)
**主要改动：**
- 使用 auth 工具保存登录信息
- 添加 checkLoginStatus() 检查是否已登录
- 优化错误处理
- 代码格式化

**关键方法：**
- `checkLoginStatus()` - 检查是否已登录，已登录则跳转首页
- `login()` - 登录逻辑，保存 Token 和用户信息

---

#### 5. `pages/mine/index.js` (172 行，已更新)
**主要改动：**
- 使用 auth 工具获取用户信息
- 优化退出登录逻辑
- 添加 goToLogin() 方法

**关键方法：**
- `checkLoginStatus()` - 检查并展示登录状态
- `loadUserInfo()` - 加载用户信息
- `logout()` - 退出登录，保留账号密码

---

#### 6. `pages/score/index.js` (已更新)
**主要改动：**
- 在 onLoad() 中添加登录检查
- 在 update() 中添加登录检查

**新增代码：**
```javascript
const auth = require("../../utils/auth");
const pageAuth = require("../../utils/pageAuth");

onLoad(options) {
  if (!pageAuth.checkAuth()) return;
  this.getList();
}
```

---

#### 7. `pages/course/index.js` (已更新)
**主要改动：**
- 在 onLoad() 中添加登录检查
- 在 updateFn() 中添加登录检查
- 代码格式化

**新增代码：**
```javascript
const auth = require("../../utils/auth");
const pageAuth = require("../../utils/pageAuth");

onLoad(options) {
  if (!pageAuth.checkAuth()) return;
  // ... 其他逻辑
}
```

---

### 文档文件（新增）

#### 8. `TOKEN_QUICK_START.md` (272 行) ⭐⭐⭐⭐⭐
**适合人群：** 所有开发者

**内容：**
- 5分钟快速上手
- 核心 API 一览
- 代码示例
- 快速测试方法

**何时阅读：**
- 首次使用时
- 需要快速参考时

---

#### 9. `TOKEN_AUTH_README.md` (212 行) ⭐⭐⭐⭐⭐
**适合人群：** 所有开发者

**内容：**
- 功能概述
- 核心文件说明
- 完整使用示例
- 注意事项

**何时阅读：**
- 了解功能全貌时
- 需要参考标准用法时

---

#### 10. `TOKEN_AUTH_GUIDE.md` (404 行) ⭐⭐⭐⭐
**适合人群：** 需要深入了解的开发者

**内容：**
- 详细 API 文档
- 多种使用场景示例
- 完整代码示例
- 常见问题解答

**何时阅读：**
- 需要实现复杂功能时
- 遇到问题需要排查时

---

#### 11. `TOKEN_TEST_GUIDE.md` (432 行) ⭐⭐⭐
**适合人群：** 测试人员、QA

**内容：**
- 完整测试场景
- 测试步骤说明
- 自动化测试脚本
- 问题排查方法

**何时阅读：**
- 进行功能测试时
- 上线前验证时

---

#### 12. `TOKEN_IMPLEMENTATION_SUMMARY.md` (437 行) ⭐⭐⭐
**适合人群：** 技术负责人、架构师

**内容：**
- 实现原理说明
- 核心逻辑流程图
- 数据流转说明
- 后续优化建议

**何时阅读：**
- 需要了解实现细节时
- 需要进行技术评审时

---

#### 13. `CHECKLIST.md` (325 行) ⭐⭐⭐⭐⭐
**适合人群：** 所有人（上线前必读）

**内容：**
- 功能检查清单
- 测试场景验证
- 代码质量检查
- 部署前确认

**何时使用：**
- 功能开发完成后
- 准备上线部署前

---

#### 14. `TOKEN_FILES_STRUCTURE.md` (本文档)
**适合人群：** 所有开发者

**内容：**
- 文件目录结构
- 各文件详细说明
- 阅读顺序建议

**何时阅读：**
- 想了解整体结构时
- 不知道该看哪个文档时

---

## 📖 推荐阅读顺序

### 🎯 快速上手（5-10分钟）
1. `TOKEN_QUICK_START.md` - 快速了解和使用
2. `TOKEN_AUTH_README.md` - 了解功能全貌

### 🔧 深入学习（30-60分钟）
1. `TOKEN_QUICK_START.md` - 快速开始
2. `TOKEN_AUTH_README.md` - 功能说明
3. `TOKEN_AUTH_GUIDE.md` - 详细指南
4. 参考已更新的页面代码

### 🧪 测试验证（1-2小时）
1. `TOKEN_QUICK_START.md` - 了解功能
2. `TOKEN_TEST_GUIDE.md` - 测试指南
3. `CHECKLIST.md` - 检查清单

### 🏗️ 技术深入（2-3小时）
1. `TOKEN_AUTH_GUIDE.md` - 详细指南
2. `TOKEN_IMPLEMENTATION_SUMMARY.md` - 实现总结
3. 阅读核心文件源码：
   - `utils/auth.js`
   - `utils/pageAuth.js`
   - `utils/request.js`

---

## 🎯 根据角色选择文档

### 开发者
**必读：**
- ⭐⭐⭐⭐⭐ `TOKEN_QUICK_START.md`
- ⭐⭐⭐⭐⭐ `TOKEN_AUTH_README.md`

**推荐：**
- ⭐⭐⭐⭐ `TOKEN_AUTH_GUIDE.md`
- ⭐⭐⭐ `TOKEN_IMPLEMENTATION_SUMMARY.md`

---

### 测试人员
**必读：**
- ⭐⭐⭐⭐⭐ `TOKEN_AUTH_README.md`
- ⭐⭐⭐⭐ `TOKEN_TEST_GUIDE.md`

**推荐：**
- ⭐⭐⭐ `TOKEN_QUICK_START.md`
- ⭐⭐⭐⭐⭐ `CHECKLIST.md`

---

### 项目负责人
**必读：**
- ⭐⭐⭐⭐⭐ `TOKEN_AUTH_README.md`
- ⭐⭐⭐⭐⭐ `CHECKLIST.md`

**推荐：**
- ⭐⭐⭐ `TOKEN_IMPLEMENTATION_SUMMARY.md`
- ⭐⭐⭐ `TOKEN_TEST_GUIDE.md`

---

### 新成员入职
**第一天：**
1. `README.md` - 了解项目
2. `TOKEN_QUICK_START.md` - 快速上手

**第一周：**
1. `TOKEN_AUTH_README.md` - 功能说明
2. `TOKEN_AUTH_GUIDE.md` - 详细指南
3. 阅读已更新的页面代码

---

## 📊 文件统计

| 类型 | 数量 | 总行数 |
|------|------|--------|
| 核心工具文件 | 3 个 | ~430 行 |
| 页面文件（已更新） | 4 个 | ~600 行 |
| 文档文件 | 7 个 | ~2300 行 |
| **总计** | **14 个** | **~3330 行** |

---

## 🔗 文件依赖关系

```
pages/*.js
    ↓ 引入
utils/auth.js + utils/pageAuth.js
    ↓ 引入
utils/request.js
    ↓ 引入
api/main.js
```

**说明：**
- 所有页面通过 `auth` 和 `pageAuth` 管理认证
- 所有 API 请求通过 `request.js` 处理
- `request.js` 自动处理 Token 失效

---

## ✅ 文件完整性检查

使用以下命令检查文件是否完整：

### Windows (PowerShell)
```powershell
# 检查核心文件
Test-Path utils\auth.js
Test-Path utils\pageAuth.js
Test-Path utils\request.js

# 检查文档文件
Test-Path TOKEN_QUICK_START.md
Test-Path TOKEN_AUTH_README.md
Test-Path TOKEN_AUTH_GUIDE.md
Test-Path TOKEN_TEST_GUIDE.md
Test-Path TOKEN_IMPLEMENTATION_SUMMARY.md
Test-Path CHECKLIST.md
```

### Linux/Mac
```bash
# 检查核心文件
ls -l utils/auth.js utils/pageAuth.js utils/request.js

# 检查文档文件
ls -l TOKEN_*.md CHECKLIST.md
```

---

## 🎉 开始使用

1. **快速入门**：阅读 `TOKEN_QUICK_START.md`
2. **参考示例**：查看已更新的页面文件
3. **遇到问题**：查看 `TOKEN_AUTH_GUIDE.md` 常见问题
4. **上线前**：使用 `CHECKLIST.md` 检查

---

## 📞 获取帮助

如果你：
- 🤔 不知道从哪个文档开始 → 看本文档的"推荐阅读顺序"
- 🚀 想快速上手 → 直接看 `TOKEN_QUICK_START.md`
- 🐛 遇到问题 → 查看 `TOKEN_AUTH_GUIDE.md` 的常见问题
- ✅ 准备上线 → 使用 `CHECKLIST.md` 逐项检查

---

**更新时间：** 2024-01
**版本：** v1.0.0