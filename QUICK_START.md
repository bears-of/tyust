# 原始成绩功能快速开始指南

## 快速测试步骤

### 1. 启动后端服务

```bash
# 进入后端目录
cd tyust_backend

# 运行后端服务
cargo run

# 或者使用 release 模式（更快）
cargo run --release
```

后端服务将在 `http://localhost:3000` 启动。

### 2. 配置微信小程序前端

#### 2.1 确认配置文件
打开 `tyust_frontend/config.js`，确认开发环境配置：

```javascript
baseUrl: {
  develop: 'http://localhost:3000/api',
  production: 'http://api.xxx.com',
}
```

#### 2.2 打开微信开发者工具
1. 打开微信开发者工具
2. 导入项目：选择 `tyust_frontend` 目录
3. 确保开启"不校验合法域名"选项（开发阶段）

### 3. 登录测试账号

在小程序中：
1. 进入登录页面
2. 输入学号和密码
3. 完成登录

### 4. 查看原始成绩

1. 导航到"成绩查询"页面
2. 点击顶部的"原始成绩"标签
3. 查看成绩列表

**功能测试**：
- ✅ 切换"有效成绩"和"原始成绩"
- ✅ 使用学期选择器切换不同学期
- ✅ 点击刷新按钮获取最新数据
- ✅ 查看成绩、学分、绩点、教师等信息

## API 测试（可选）

### 使用 curl 测试后端接口

```bash
# 首先登录获取 token
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"student_id":"你的学号","password":"你的密码"}'

# 使用返回的 token 查询原始成绩
curl -X GET http://localhost:3000/api/raw-scores \
  -H "token: 你的token" \
  -H "studentId: 你的学号"

# 查询特定学年的成绩
curl -X GET "http://localhost:3000/api/raw-scores?xnm=2024" \
  -H "token: 你的token" \
  -H "studentId: 你的学号"

# 查询特定学期的成绩
curl -X GET "http://localhost:3000/api/raw-scores?xnm=2024&xqm=12" \
  -H "token: 你的token" \
  -H "studentId: 你的学号"
```

### 使用 Postman/Insomnia 测试

1. **登录接口**
   - Method: POST
   - URL: `http://localhost:3000/api/auth/login`
   - Body (JSON):
     ```json
     {
       "student_id": "你的学号",
       "password": "你的密码"
     }
     ```

2. **原始成绩接口**
   - Method: GET
   - URL: `http://localhost:3000/api/raw-scores`
   - Headers:
     - `token`: 从登录接口获取的 token
     - `studentId`: 你的学号

## 预期结果

### 成功响应示例

```json
{
  "code": 0,
  "message": "success",
  "data": [
    {
      "semester": "2024-2025 2",
      "course": "数字图像处理实验",
      "credit": "0.5",
      "score": "良",
      "gpa": "3.50",
      "teacher": "李东红",
      "courseType": "教学环节"
    },
    {
      "semester": "2024-2025 2",
      "course": "嵌入式系统课程设计",
      "credit": "2.0",
      "score": "良",
      "gpa": "3.50",
      "teacher": "乔建华",
      "courseType": "教学环节"
    }
  ]
}
```

### 小程序界面预期效果

```
┌─────────────────────────────────────┐
│ [有效成绩] [原始成绩]  [选择学期 ▼] [🔄] │
├─────────────────────────────────────┤
│ 2024-2025 2                         │
│                                     │
│ ┌─────────────────────────────────┐ │
│ │ 数字图像处理实验           良   │ │
│ │ 学分: 0.5  绩点: 3.50  教师: 李东红 │ │
│ │ [教学环节]                      │ │
│ └─────────────────────────────────┘ │
│                                     │
│ ┌─────────────────────────────────┐ │
│ │ 嵌入式系统课程设计         良   │ │
│ │ 学分: 2.0  绩点: 3.50  教师: 乔建华 │ │
│ │ [教学环节]                      │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

## 常见问题

### 1. 后端编译错误
```bash
# 更新依赖
cargo update

# 清理并重新构建
cargo clean
cargo build
```

### 2. 前端无法连接后端
- 检查后端服务是否启动（端口 3000）
- 检查 `config.js` 中的 baseUrl 配置
- 确保微信开发者工具中开启"不校验合法域名"

### 3. 登录失败
- 检查学号和密码是否正确
- 查看后端日志是否有错误信息
- 确认数据库连接正常

### 4. 成绩数据为空
- 确认该学号确实有成绩数据
- 检查网络请求是否成功（微信开发者工具 Network 面板）
- 查看控制台是否有错误信息

### 5. Token 过期
- 重新登录获取新的 token
- 检查后端的 token 有效期设置

## 开发调试

### 后端日志
后端会输出详细的日志信息，包括：
- API 请求
- 数据库操作
- 错误信息

### 前端调试
在微信开发者工具的控制台中：
```javascript
// 查看缓存数据
console.log(wx.getStorageSync('rawScores'))

// 清除缓存
wx.removeStorageSync('rawScores')
wx.removeStorageSync('scores')

// 查看 token
console.log(wx.getStorageSync('token'))
console.log(wx.getStorageSync('studentId'))
```

## 下一步

1. **功能测试**: 按照 `RAW_SCORES_INTEGRATION.md` 中的测试用例进行完整测试
2. **性能测试**: 测试大量数据时的加载性能
3. **兼容性测试**: 在不同手机/微信版本上测试
4. **部署**: 将后端部署到生产服务器，更新前端配置

## 相关文档

- 📄 `INTEGRATION_SUMMARY.md` - 完整的集成总结
- 📄 `tyust_backend/CHANGES_RAW_SCORES.md` - 后端修改详情
- 📄 `tyust_frontend/RAW_SCORES_INTEGRATION.md` - 前端集成文档

## 技术支持

遇到问题？
1. 查看相关文档
2. 检查日志输出
3. 提交 Issue 或 Pull Request

---

**祝测试顺利！** 🎉