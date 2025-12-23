# 原始成绩功能集成总结

## 项目概述
本次任务是将后端的原始成绩接口与微信小程序前端进行完整集成，使用户能够在小程序中查看原始成绩信息。

## 完成的工作

### 一、后端适配 (Rust)

#### 1. 更新 `tyust_get_raw_scores` 函数
**文件**: `tyust_backend/src/tyust_api.rs`

**主要变更**:
- 修改函数签名，移除 `access_token` 参数
- 添加 `xh_id`（学号ID）、`xnm`（学年码）、`xqm`（学期码）参数
- 更改 API 端点：`cjcx_cxXsYscj.html` → `cjcx_cxDgXscj.html`
- 更改参数：`gnmkdm: N305007` → `N305005`，添加 `doType: query`
- 更新表单数据：
  - 添加 `xh_id` 字段
  - `queryModel.sortName` 从空字符串改为单个空格
  - `time` 从 "1" 改为 "0"
- 移除 `__access_token` cookie，仅保留 `JSESSIONID` 和 `route`

```rust
// 修改前
pub async fn tyust_get_raw_scores(
    jwglxt_jsession: &str,
    access_token: &str,
    route: &str,
) -> Result<Vec<entity::ScoreItem>>

// 修改后
pub async fn tyust_get_raw_scores(
    jwglxt_jsession: &str,
    route: &str,
    xh_id: &str,
    xnm: &str,
    xqm: &str,
) -> Result<Vec<entity::ScoreItem>>
```

#### 2. 处理不同的 API 响应结构
**文件**: `tyust_backend/src/entity.rs`

**问题**: 新端点返回的 JSON 结构与旧端点不同，许多字段缺失导致反序列化失败。

**解决方案**:
- 为 `ScoreItem` 结构体的约 50 个字段添加 `#[serde(default)]` 属性
- 为 `QueryModel` 和 `UserModel` 添加 `Default` trait
- 为 `TyustScoreResponse` 添加 `#[serde(flatten)]` 以接收额外的分页字段

**关键字段**（必需）:
- `cj` - 成绩
- `jd` - 绩点
- `kcmc` - 课程名称
- `xf` - 学分
- `xh` - 学号
- `xnm`, `xnmmc` - 学年
- `xqm`, `xqmmc` - 学期

#### 3. 更新 Handler 和 API 类型
**文件**: `tyust_backend/src/handlers.rs`, `tyust_backend/src/api_types.rs`

**变更**:
- 添加 `RawScoresParams` 结构体用于接收查询参数：
  ```rust
  pub struct RawScoresParams {
      pub xh_id: Option<String>,
      pub xnm: Option<String>,
      pub xqm: Option<String>,
  }
  ```
- 更新 `get_raw_scores` handler 接收 Query 参数
- 设置默认值：`xh_id` 默认为当前用户学号，`xnm` 和 `xqm` 默认为空字符串

#### 4. API 响应格式
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
    }
  ]
}
```

### 二、前端适配 (微信小程序)

#### 1. 数据转换逻辑
**文件**: `tyust_frontend/pages/score/index.js`

**核心功能**: `transformScoreData()` 方法

**作用**:
- 将后端返回的扁平数组转换为按学期分组的结构
- 映射字段名称（`course` → `name`）
- 为原始成绩添加 `complexScore` 字段
- 按学期倒序排序（最新学期在前）

**数据流**:
```
后端 → 扁平数组 → transformScoreData() → 分组数组 → 前端显示
```

#### 2. UI 布局优化
**文件**: `tyust_frontend/pages/score/index.wxml`

**新增显示内容**:
- 课程名称（顶部，加粗）
- 成绩/综合成绩（右侧，大号字体）
- 详细信息行：
  - 学分
  - 绩点
  - 教师姓名
- 课程类型标签
- 原始成绩详情（平时、期中、期末 - 预留）

**布局结构**:
```
┌──────────────────────────────────┐
│ 课程名称                    85   │
│ 学分: 2.0  绩点: 3.5  教师: XX  │
│ [课程类型标签]                   │
└──────────────────────────────────┘
```

#### 3. 样式更新
**文件**: `tyust_frontend/pages/score/index.wxss`

**优化内容**:
- 使用卡片式设计
- 添加课程详情样式（`.course-details`, `.detail-item`）
- 添加课程类型标签样式（`.course-type`）
- 改进布局为纵向排列，更好地展示信息
- 响应式字体大小和间距

#### 4. 功能特性

**成绩类型切换**:
- 有效成绩：显示 `score` 字段
- 原始成绩：显示 `complexScore` 字段（若无则显示 `score`）

**学期选择**:
- 使用 picker 组件
- 显示所有可用学期
- 支持快速切换

**数据缓存**:
- 有效成绩缓存 key: `scores`
- 原始成绩缓存 key: `rawScores`
- 切换时优先读取缓存
- 刷新时更新缓存

**刷新功能**:
- 显示加载动画
- 成功后显示提示
- 错误时显示错误信息

### 三、文档

#### 1. 后端文档
**文件**: `tyust_backend/CHANGES_RAW_SCORES.md`

**内容**:
- 详细的函数签名变更
- API 端点和参数变化
- 数据结构修改说明
- 可选字段列表
- 使用示例

#### 2. 前端文档
**文件**: `tyust_frontend/RAW_SCORES_INTEGRATION.md`

**内容**:
- API 接口说明
- 数据流程图
- UI 布局设计
- 完整测试用例
- 已知问题和限制
- 后续优化建议

## 技术要点

### 1. 后端
- **Rust/Axum**: Web 框架
- **Serde**: JSON 序列化/反序列化，使用 `#[serde(default)]` 处理可选字段
- **请求封装**: 使用 reqwest 进行 HTTP 请求
- **错误处理**: 使用 anyhow::Result

### 2. 前端
- **微信小程序**: WXML + WXSS + JavaScript
- **数据转换**: Map/Array 操作进行分组和排序
- **缓存策略**: wx.getStorageSync/wx.setStorageSync
- **请求封装**: 统一的 createRequest 函数

## API 使用示例

### 后端 API

```bash
# 获取所有成绩
GET /api/raw-scores

# 获取指定学年成绩
GET /api/raw-scores?xnm=2024

# 获取指定学期成绩
GET /api/raw-scores?xnm=2024&xqm=12

# 查询特定学生成绩
GET /api/raw-scores?xh_id=202112181110
```

### 前端调用

```javascript
// 获取原始成绩
import { getRawScoreListRequest } from '../../api/main'

getRawScoreListRequest()
  .then(res => {
    const data = this.transformScoreData(res.data, true)
    // 处理数据
  })
```

## 测试验证

### 后端测试
```bash
cd tyust_backend
cargo test test_get_user_raw_scores
```

### 前端测试
1. 启动后端服务：`cargo run`
2. 打开微信开发者工具
3. 导航到"成绩查询"页面
4. 验证数据正确显示

### 测试清单
- [x] 后端接口返回正确数据
- [x] 前端正确解析数据
- [x] 学期分组正确
- [x] 成绩显示正常
- [x] 学分、绩点、教师信息显示
- [x] 缓存功能正常
- [x] 刷新功能正常
- [x] 切换成绩类型正常
- [x] 样式美观

## 已知限制

### 1. 详细成绩分数
后端当前只返回总成绩，不包括：
- 平时成绩 (normalScore)
- 期中成绩 (midtermScore)
- 期末成绩 (finalScore)

这些字段在前端预留但暂时为空。

### 2. 成绩格式
- 数字成绩：85, 90, 66
- 等级成绩：优, 良, 中, 及格, 不及格

前端对两种格式都做了处理。

### 3. 及格判断
- 数字 < 60：显示为红色
- 等级成绩：显示为蓝色（默认及格）

## 后续优化方向

### 功能增强
- 成绩筛选（按课程类型、及格状态等）
- 成绩统计（平均分、总学分、GPA）
- 成绩趋势图表
- 导出成绩单

### 性能优化
- 虚拟列表（大数据量）
- 增量更新
- 请求去重
- 智能缓存

### 用户体验
- 骨架屏加载
- 下拉刷新
- 成绩详情页
- 分享功能

## 相关文件清单

### 后端
```
tyust_backend/
├── src/
│   ├── tyust_api.rs          # API 调用实现 ✓
│   ├── handlers.rs            # 路由处理器 ✓
│   ├── entity.rs              # 数据结构 ✓
│   ├── api_types.rs           # API 类型 ✓
│   └── main.rs                # 主程序
├── CHANGES_RAW_SCORES.md     # 变更文档 ✓
└── Cargo.toml
```

### 前端
```
tyust_frontend/
├── pages/score/
│   ├── index.js               # 页面逻辑 ✓
│   ├── index.wxml             # 页面结构 ✓
│   ├── index.wxss             # 页面样式 ✓
│   └── index.json
├── api/
│   └── main.js                # API 调用
├── utils/
│   └── request.js             # 请求封装
├── RAW_SCORES_INTEGRATION.md  # 集成文档 ✓
└── config.js
```

## 总结

本次集成工作成功实现了：
1. ✅ 后端 Rust API 适配 Python 实现
2. ✅ 处理不同 API 响应结构的兼容性问题
3. ✅ 前端数据转换和展示逻辑
4. ✅ 完整的 UI/UX 优化
5. ✅ 详细的文档和测试用例

系统现在可以完整地展示用户的原始成绩信息，包括成绩、学分、绩点、教师等详细信息，并支持按学期分组查看和切换。