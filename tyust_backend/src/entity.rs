use serde::{Deserialize, Serialize};

/// 完整的用户信息（包含token等额外字段）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserLoginInfo {
    #[serde(rename = "studentId")]
    pub student_id: String,
    pub name: String,
    pub class: String,
    pub token: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>, // 头像URL
}

impl UserLoginInfo {
    /// 创建新的用户登录信息
    pub fn new(student_id: String, name: String, class: String, token: String) -> Self {
        Self {
            student_id,
            name,
            class,
            token,
            avatar_url: None,
        }
    }

    /// 设置头像URL
    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
    }
}

/// 太原科技大学课表响应结构
#[derive(Debug, Deserialize, Serialize)]
pub struct TyustCourseResponse {
    #[serde(rename = "qsxqj")]
    pub qsxqj: String, // 起始星期几
    pub xsxx: Xsxx, // 学生信息
    #[serde(rename = "sjkList")]
    pub sjk_list: Vec<Sjk>, // 实践课列表
    #[serde(rename = "xqjmcMap")]
    pub xqjmc_map: std::collections::HashMap<String, String>, // 星期几名称映射
    pub xskbsfxstkzt: String, // 学生课表是否显示停课状态
    #[serde(rename = "rqazcList")]
    pub rqazc_list: Vec<serde_json::Value>, // 日期安排冲突列表
    #[serde(rename = "kbList")]
    pub kb_list: Vec<Kb>, // 课表列表
}

/// 学生信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Xsxx {
    #[serde(rename = "BJMC")]
    pub bjmc: String, // 班级名称
    #[serde(rename = "XNMC")]
    pub xnmc: String, // 学年名称
    #[serde(rename = "KXKXXQ")]
    pub kxkxxq: String, // 可选课学期
    #[serde(rename = "XKKGXQ")]
    pub xkkgxq: String, // 选课开关学期
    #[serde(rename = "XKKG")]
    pub xkkg: String, // 选课开关
    #[serde(rename = "ZYH_ID")]
    pub zyh_id: String, // 专业号ID
    #[serde(rename = "XH_ID")]
    pub xh_id: String, // 学号ID
    #[serde(rename = "XH")]
    pub xh: String, // 学号
    #[serde(rename = "XQMMC")]
    pub xqmmc: String, // 校区名称
    #[serde(rename = "JFZT")]
    pub jfzt: i32, // 缴费状态
    #[serde(rename = "XM")]
    pub xm: String, // 姓名
    #[serde(rename = "XQM")]
    pub xqm: String, // 学期码
    #[serde(rename = "XNM")]
    pub xnm: String, // 学年码
    #[serde(rename = "NJDM_ID")]
    pub njdm_id: String, // 年级代码ID
    #[serde(rename = "JSXM")]
    pub jsxm: String, // 教师姓名
    #[serde(rename = "KCMS")]
    pub kcms: i32, // 课程门数
    #[serde(rename = "ZYMC")]
    pub zymc: String, // 专业名称
}

/// 实践课信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Sjk {
    pub cxbj: String, // 重修标记
    pub date: String, // 日期
    #[serde(rename = "dateDigit")]
    pub date_digit: String, // 数字日期
    #[serde(rename = "dateDigitSeparator")]
    pub date_digit_separator: String, // 分隔符日期
    pub day: String,  // 日
    pub jgpxzd: String, // 机构排序字段
    pub jsxm: String, // 教师姓名
    pub jxbzh: String, // 教学班组合
    pub kclb: String, // 课程类别
    pub kcmc: String, // 课程名称
    pub listnav: String, // 列表导航
    #[serde(rename = "localeKey")]
    pub locale_key: String, // 本地化键
    pub month: String, // 月
    #[serde(rename = "pageTotal")]
    pub page_total: i32, // 页面总数
    pub pageable: bool, // 可分页
    pub qsjsz: String, // 起始结束周
    pub qtkcgs: String, // 其他课程概述
    #[serde(rename = "queryModel")]
    pub query_model: QueryModel, // 查询模型
    pub rangeable: bool, // 可范围查询
    pub rsdzjs: i32,  // 人数达到教师数
    pub sjkcgs: String, // 实践课程概述
    #[serde(rename = "totalResult")]
    pub total_result: String, // 总结果
    #[serde(rename = "userModel")]
    pub user_model: UserModel, // 用户模型
    pub xf: String,   // 学分
    pub xksj: String, // 选课时间
    pub xnmc: String, // 学年名称
    pub xqmc: String, // 校区名称
    pub xqmmc: String, // 校区名称码
    pub year: String, // 年份
}

/// 课表信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Kb {
    pub bklxdjmc: String, // 本科类型等级名称
    pub cd_id: String,    // 场地ID
    pub cdlbmc: String,   // 场地类别名称
    pub cdmc: String,     // 场地名称
    pub cxbj: String,     // 重修标记
    pub cxbjmc: String,   // 重修标记名称
    pub date: String,     // 日期
    #[serde(rename = "dateDigit")]
    pub date_digit: String, // 数字日期
    #[serde(rename = "dateDigitSeparator")]
    pub date_digit_separator: String, // 分隔符日期
    pub day: String,      // 日
    pub jc: String,       // 节次
    pub jcor: String,     // 节次或
    pub jcs: String,      // 节次s
    pub jgh_id: String,   // 教工号ID
    pub jgpxzd: String,   // 机构排序字段
    pub jxb_id: String,   // 教学班ID
    pub jxbmc: String,    // 教学班名称
    pub jxbsftkbj: String, // 教学班是否停课标记
    pub jxbzc: String,    // 教学班组成
    pub kcbj: String,     // 课程标记
    pub kch: String,      // 课程号
    pub kch_id: String,   // 课程号ID
    pub kclb: String,     // 课程类别
    pub kcmc: String,     // 课程名称
    pub kcxszc: String,   // 课程学时组成
    pub kcxz: String,     // 课程性质
    pub kczxs: String,    // 课程总学时
    pub khfsmc: String,   // 考核方式名称
    pub kkzt: String,     // 开课状态
    pub lh: String,       // 楼号
    pub listnav: String,  // 列表导航
    #[serde(rename = "localeKey")]
    pub locale_key: String, // 本地化键
    pub month: String,    // 月份
    pub oldjc: String,    // 原始节次
    pub oldzc: String,    // 原始周次
    #[serde(rename = "pageTotal")]
    pub page_total: i32, // 页面总数
    pub pageable: bool,   // 可分页
    pub pkbj: String,     // 排课标记
    pub px: String,       // 排序
    pub qqqh: String,     // 其他信息
    #[serde(rename = "queryModel")]
    pub query_model: QueryModel, // 查询模型
    pub rangeable: bool,  // 可范围查询
    pub rk: String,       // 容量
    pub rsdzjs: i32,      // 人数达到教师数
    pub sfjf: String,     // 是否缴费
    pub skfsmc: String,   // 授课方式名称
    pub sxbj: String,     // 实习标记
    #[serde(rename = "totalResult")]
    pub total_result: String, // 总结果
    #[serde(rename = "userModel")]
    pub user_model: UserModel, // 用户模型
    pub xf: String,       // 学分
    pub xkbz: String,     // 选课备注
    pub xm: String,       // 教师姓名
    pub xnm: String,      // 学年码
    pub xqdm: String,     // 校区代码
    pub xqh1: String,     // 校区号1
    pub xqh_id: String,   // 校区号ID
    pub xqj: String,      // 星期几
    pub xqjmc: String,    // 星期几名称
    pub xqm: String,      // 学期码
    pub xqmc: String,     // 校区名称
    pub xsdm: String,     // 学生代码
    pub xslxbj: String,   // 学生类型标记
    pub year: String,     // 年份
    pub zcd: String,      // 周次段
    pub zcmc: String,     // 职称名称
    pub zfjmc: String,    // 主副讲名称
    pub zhxs: String,     // 综合学时
    pub zxs: String,      // 总学时
    pub zxxx: String,     // 执行信息
    pub zyfxmc: String,   // 专业方向名称
    pub zyhxkcbj: String, // 专业核心课程标记
    pub zzmm: String,     // 政治面貌
    pub zzrl: String,     // 总人数
}

/// 查询模型
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct QueryModel {
    #[serde(rename = "currentPage")]
    pub current_page: i32, // 当前页
    #[serde(rename = "currentResult")]
    pub current_result: i32, // 当前结果
    #[serde(rename = "entityOrField")]
    pub entity_or_field: bool, // 实体或字段
    pub limit: i32,  // 限制
    pub offset: i32, // 偏移
    #[serde(rename = "pageNo")]
    pub page_no: i32, // 页码
    #[serde(rename = "pageSize")]
    pub page_size: i32, // 页面大小
    #[serde(rename = "showCount")]
    pub show_count: i32, // 显示数量
    pub sorts: Vec<serde_json::Value>, // 排序
    #[serde(rename = "totalCount")]
    pub total_count: i32, // 总数量
    #[serde(rename = "totalPage")]
    pub total_page: i32, // 总页数
    #[serde(rename = "totalResult")]
    pub total_result: i32, // 总结果
}

/// 用户模型
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UserModel {
    pub monitor: bool, // 监控
    #[serde(rename = "roleCount")]
    pub role_count: i32, // 角色数量
    #[serde(rename = "roleKeys")]
    pub role_keys: String, // 角色键
    #[serde(rename = "roleValues")]
    pub role_values: String, // 角色值
    pub status: i32,   // 状态
    pub usable: bool,  // 可用
}

/// 融合门户用户信息响应
#[derive(Debug, Deserialize, Serialize)]
pub struct RonghemenhuUserInfoResponse {
    pub code: i32,
    #[serde(rename = "msg")]
    pub message: String,
    pub data: RonghemenhuUserInfo,
}

/// 融合门户用户信息
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RonghemenhuUserInfo {
    pub acad: String, // 学号
    #[serde(rename = "baseMenuList")]
    pub base_menu_list: Vec<serde_json::Value>, // 基础菜单列表
    pub card: String, // 卡号
    pub ctime: i64,   // 创建时间
    pub email: String, // 邮箱
    pub enable: i32,  // 启用状态
    pub id: i64,      // 用户ID
    #[serde(rename = "isImproveInfo")]
    pub is_improve_info: i32, // 是否完善信息
    pub lgid: i64,    // 登录ID
    pub name: String, // 姓名
    pub phone: String, // 手机号
    #[serde(rename = "roleSign")]
    pub role_sign: Vec<String>, // 角色标识
    pub sex: i32,     // 性别
    pub stage: i32,   // 阶段
    pub ucode: String, // 用户代码
    #[serde(rename = "userType")]
    pub user_type: i32, // 用户类型
    pub utime: i64,   // 更新时间
    #[serde(rename = "uvCode")]
    pub uv_code: i64, // UV代码
}

/// 成绩查询响应
#[derive(Debug, Deserialize, Serialize)]
pub struct TyustScoreResponse {
    pub items: Vec<ScoreItem>, // 成绩列表
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>, // 其他字段（如分页信息）
}

/// 成绩项
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScoreItem {
    #[serde(default)]
    pub bfzcj: String, // 百分制成绩
    #[serde(default)]
    pub bh: String, // 编号
    #[serde(default)]
    pub bh_id: String, // 编号ID
    #[serde(default)]
    pub bj: String, // 班级
    pub cj: String, // 成绩
    #[serde(default)]
    pub cjbdczr: String, // 成绩变动操作人
    #[serde(default)]
    pub cjbdsj: String, // 成绩变动时间
    #[serde(default)]
    pub cjsfzf: String, // 成绩是否作废
    #[serde(default)]
    pub date: String, // 日期
    #[serde(rename = "dateDigit", default)]
    pub date_digit: String, // 数字日期
    #[serde(rename = "dateDigitSeparator", default)]
    pub date_digit_separator: String, // 分隔符日期
    #[serde(default)]
    pub day: String, // 日
    pub jd: String, // 绩点
    #[serde(default)]
    pub jg_id: String, // 机构ID
    pub jgmc: String, // 机构名称
    #[serde(default)]
    pub jgpxzd: String, // 机构排序字段
    pub jsxm: String, // 教师姓名
    #[serde(default)]
    pub jxb_id: String, // 教学班ID
    #[serde(default)]
    pub jxbmc: String, // 教学班名称
    #[serde(default)]
    pub kcbj: String, // 课程标记
    #[serde(default)]
    pub kcgsmc: Option<String>, // 课程归属名称
    pub kch: String, // 课程号
    pub kch_id: String, // 课程号ID
    pub kclbmc: String, // 课程类别名称
    pub kcmc: String, // 课程名称
    #[serde(default)]
    pub kcxzdm: String, // 课程性质代码
    pub kcxzmc: String, // 课程性质名称
    #[serde(default)]
    pub key: String, // 键
    #[serde(default)]
    pub khfsmc: String, // 考核方式名称
    #[serde(default)]
    pub kkbmmc: String, // 开课部门名称
    #[serde(default)]
    pub ksxz: String, // 考试性质
    #[serde(default)]
    pub ksxzdm: String, // 考试性质代码
    #[serde(default)]
    pub listnav: String, // 列表导航
    #[serde(rename = "localeKey", default)]
    pub locale_key: String, // 本地化键
    #[serde(default)]
    pub month: String, // 月
    #[serde(default)]
    pub njdm_id: String, // 年级代码ID
    #[serde(default)]
    pub njmc: String, // 年级名称
    #[serde(rename = "pageTotal", default)]
    pub page_total: i32, // 页面总数
    #[serde(default)]
    pub pageable: bool, // 可分页
    #[serde(rename = "queryModel", default)]
    pub query_model: QueryModel, // 查询模型
    #[serde(default)]
    pub rangeable: bool, // 可范围查询
    #[serde(default)]
    pub row_id: String, // 行ID
    #[serde(default)]
    pub rwzxs: String, // 任务总学时
    #[serde(default)]
    pub sfdkbcx: String, // 是否打开补充
    #[serde(default)]
    pub sfxwkc: String, // 是否学位课程
    #[serde(default)]
    pub sfzh: String, // 身份证号
    #[serde(default)]
    pub sfzx: String, // 是否在校
    #[serde(default)]
    pub tjrxm: String, // 提交人姓名
    #[serde(default)]
    pub tjsj: String, // 提交时间
    #[serde(rename = "totalResult", default)]
    pub total_result: String, // 总结果
    #[serde(rename = "userModel", default)]
    pub user_model: UserModel, // 用户模型
    #[serde(default)]
    pub xb: String, // 性别
    #[serde(default)]
    pub xbm: String, // 性别码
    pub xf: String, // 学分
    pub xfjd: String, // 学分绩点
    pub xh: String, // 学号
    pub xh_id: String, // 学号ID
    pub xm: String, // 姓名
    pub xnm: String, // 学年码
    pub xnmmc: String, // 学年名称
    pub xqm: String, // 学期码
    pub xqmmc: String, // 学期名称
    #[serde(default)]
    pub xz: String, // 学制
    #[serde(default)]
    pub year: String, // 年份
    #[serde(default)]
    pub zxs: String, // 总学时
    pub zyh_id: String, // 专业号ID
    pub zymc: String, // 专业名称
}
