# 原始成绩功能集成文档

## 概述
本文档说明了微信小程序前端与后端原始成绩接口的集成情况。

## 后端接口

### 1. 原始成绩接口
- **URL**: `GET /api/raw-scores`
- **需要认证**: 是
- **查询参数** (可选):
  - `xh_id`: 学号ID（默认为当前登录用户）
  - `xnm`: 学年码（如"2024"，空字符串表示所有学年）
  - `xqm`: 学期码（如"3"，空字符串表示所有学期）

### 2. 请求示例
```bash
# 获取所有成绩
GET /api/raw-scores

# 获取特定学年的成绩
GET /api/raw-scores?xnm=2024

# 获取特定学期的成绩
GET /api/raw-scores?xnm=2024&xqm=12
```

### 3. 响应格式
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

## 前端实现

### 1. 文件结构
```
pages/score/
├── index.js       # 页面逻辑
├── index.wxml     # 页面结构
├── index.wxss     # 页面样式
└── index.json     # 页面配置
```

### 2. 数据流程

#### 2.1 数据获取
前端通过 `getRawScoreListRequest()` 调用后端接口：
```javascript
// api/main.js
export function getRawScoreListRequest(data) {
  return createRequest({
    url: '/raw-scores',
    data
  })
}
```

#### 2.2 数据转换
后端返回的扁平数组会被转换为按学期分组的格式：

**后端格式**:
```javascript
[
  { semester: "2024-2025 2", course: "课程1", score: "85", ... },
  { semester: "2024-2025 2", course: "课程2", score: "90", ... },
  { semester: "2023-2024 1", course: "课程3", score: "88", ... }
]
```

**前端格式**:
```javascript
[
  {
    termName: "2024-2025 2",
    scoreList: [
      { name: "课程1", score: "85", complexScore: "85", ... },
      { name: "课程2", score: "90", complexScore: "90", ... }
    ]
  },
  {
    termName: "2023-2024 1",
    scoreList: [
      { name: "课程3", score: "88", complexScore: "88", ... }
    ]
  }
]
```

#### 2.3 数据转换逻辑
```javascript
transformScoreData(backendData, isRaw = false) {
  // 1. 按学期分组
  // 2. 转换字段名称（course -> name）
  // 3. 对原始成绩添加 complexScore 字段
  // 4. 按学期排序（最新在前）
}
```

### 3. 页面功能

#### 3.1 成绩类型切换
- **有效成绩**: 显示 `score` 字段
- **原始成绩**: 显示 `complexScore` 字段（如果不存在则显示 `score`）

#### 3.2 学期选择
使用 picker 组件选择不同学期的成绩

#### 3.3 刷新功能
点击刷新按钮重新获取最新成绩数据

#### 3.4 数据缓存
- 有效成绩缓存 key: `scores`
- 原始成绩缓存 key: `rawScores`
- 切换成绩类型时优先读取缓存

### 4. UI 布局

#### 4.1 顶部操作栏
```
[有效成绩] [原始成绩]  [选择学期 ▼]  [🔄]
```

#### 4.2 成绩列表项
```
课程名称                        成绩
学分: 2.0  绩点: 3.5  教师: 张三
[课程类型标签]
```

对于原始成绩，额外显示：
```
平时: 85  期中: 90  期末: 92
```
（注：当前后端暂不返回详细成绩，这部分为空）

## 测试步骤

### 1. 本地开发环境测试

#### 1.1 启动后端服务
```bash
cd tyust_backend
cargo run
```

#### 1.2 配置前端
确保 `config.js` 中的 `develop` URL 正确：
```javascript
baseUrl: {
  develop: 'http://localhost:3000/api',
}
```

#### 1.3 打开微信开发者工具
1. 导入项目 `tyust_frontend`
2. 确保已登录（有 token 和 studentId）
3. 导航到"成绩查询"页面

### 2. 测试用例

#### 2.1 基本功能测试
- [ ] 页面能正常加载
- [ ] 默认显示"有效成绩"
- [ ] 能切换到"原始成绩"
- [ ] 学期选择器显示正确
- [ ] 成绩列表正确显示

#### 2.2 数据显示测试
- [ ] 课程名称正确显示
- [ ] 成绩/综合成绩正确显示
- [ ] 学分显示正确
- [ ] 绩点显示正确
- [ ] 教师姓名显示正确
- [ ] 课程类型标签显示正确

#### 2.3 交互测试
- [ ] 切换成绩类型后数据正确更新
- [ ] 选择不同学期后数据正确更新
- [ ] 点击刷新按钮能获取最新数据
- [ ] 刷新时显示加载动画
- [ ] 刷新完成后显示提示信息

#### 2.4 缓存测试
- [ ] 首次加载后数据被缓存
- [ ] 关闭页面重新进入时读取缓存
- [ ] 切换成绩类型时使用各自的缓存
- [ ] 刷新后缓存被更新

#### 2.5 异常处理测试
- [ ] 未登录时跳转到登录页
- [ ] 网络错误时显示错误提示
- [ ] 没有成绩数据时显示提示信息
- [ ] token 过期时提示重新登录

### 3. 数据验证

#### 3.1 网络请求验证
在微信开发者工具的"调试器" -> "Network"中查看：
- 请求 URL: `http://localhost:3000/api/raw-scores`
- 请求方法: `GET`
- 请求头包含: `token`, `studentId`
- 响应状态码: `200`
- 响应格式符合预期

#### 3.2 控制台日志
在 `index.js` 的 `update()` 方法中查看：
```javascript
console.log('原始响应:', res.data)
console.log('转换后数据:', transformedData)
```

## 已知问题与限制

### 1. 详细成绩字段
当前后端只返回总成绩，不包括：
- 平时成绩 (normalScore)
- 期中成绩 (midtermScore)
- 期末成绩 (finalScore)

如需这些字段，需要后端提供更详细的数据。

### 2. 成绩格式
成绩可能是：
- 数字格式：85, 90, 66
- 等级格式：优, 良, 中, 及格, 不及格

前端需要正确处理这两种格式。

### 3. 及格线判断
- 数字成绩：< 60 为不及格（红色显示）
- 等级成绩：目前统一按及格处理（蓝色显示）

## 后续优化建议

### 1. 功能增强
- [ ] 添加成绩筛选（按课程类型、及格/不及格等）
- [ ] 添加成绩统计（平均分、总学分、平均绩点等）
- [ ] 支持导出成绩单
- [ ] 添加成绩趋势图表

### 2. 性能优化
- [ ] 实现增量更新（只更新变化的数据）
- [ ] 添加下拉刷新和上拉加载
- [ ] 优化大数据量时的渲染性能

### 3. 用户体验
- [ ] 添加骨架屏加载效果
- [ ] 优化成绩项的展开/收起动画
- [ ] 添加成绩详情页面
- [ ] 支持长按复制课程信息

### 4. 数据查询
如果后端支持查询参数，可以添加：
- 按学年筛选：`?xnm=2024`
- 按学期筛选：`?xnm=2024&xqm=12`
- 自定义学号查询：`?xh_id=202112181110`

示例实现：
```javascript
// 在 api/main.js 中
export function getRawScoreListRequest(params = {}) {
  return createRequest({
    url: '/raw-scores',
    data: params  // { xnm: '2024', xqm: '12' }
  })
}

// 在页面中使用
getRawScoreListRequest({ xnm: '2024', xqm: '12' })
```

## 相关文件

### 后端
- `tyust_backend/src/tyust_api.rs` - API 调用实现
- `tyust_backend/src/handlers.rs` - 路由处理器
- `tyust_backend/src/entity.rs` - 数据结构定义
- `tyust_backend/src/api_types.rs` - API 类型定义
- `tyust_backend/CHANGES_RAW_SCORES.md` - 后端修改文档

### 前端
- `tyust_frontend/pages/score/index.js` - 页面逻辑
- `tyust_frontend/pages/score/index.wxml` - 页面结构
- `tyust_frontend/pages/score/index.wxss` - 页面样式
- `tyust_frontend/api/main.js` - API 调用
- `tyust_frontend/utils/request.js` - 请求封装

## 联系与支持

如有问题或建议，请提交 Issue 或 Pull Request。