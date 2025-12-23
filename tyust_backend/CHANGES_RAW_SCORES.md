# Changes to tyust_get_raw_scores Function

## Summary
Adapted the `tyust_get_raw_scores` function in `tyust_api.rs` to match the Python implementation in `get_user_scores`. The main changes involve using a different API endpoint and adding support for custom query parameters.

## Changes Made

### 1. Function Signature (`tyust_api.rs`)
**Before:**
```rust
pub async fn tyust_get_raw_scores(
    jwglxt_jsession: &str,
    access_token: &str,
    route: &str,
) -> Result<Vec<entity::ScoreItem>>
```

**After:**
```rust
pub async fn tyust_get_raw_scores(
    jwglxt_jsession: &str,
    route: &str,
    xh_id: &str,
    xnm: &str,
    xqm: &str,
) -> Result<Vec<entity::ScoreItem>>
```

**Key Changes:**
- Removed `access_token` parameter (not used in Python version)
- Added `xh_id` (student ID) parameter
- Added `xnm` (学年码 - academic year code) parameter
- Added `xqm` (学期码 - semester code) parameter

### 2. API Endpoint Changes
**Before:**
- URL: `https://newjwc.tyust.edu.cn/jwglxt/cjcx/cjcx_cxXsYscj.html`
- gnmkdm: `N305007`
- Referer: `cjcx_cxXsYscj.html?gnmkdm=N305007`

**After:**
- URL: `https://newjwc.tyust.edu.cn/jwglxt/cjcx/cjcx_cxDgXscj.html`
- gnmkdm: `N305005`
- Added doType: `query`
- Referer: `xsxxxggl/xsgrxxwh_cxXsgrxx.html?gnmkdm=N100801`

### 3. Form Data Changes
**Before:**
```rust
let form = [
    ("xnm", ""),
    ("xqm", ""),
    ("_search", "false"),
    ("nd", &timestamp),
    ("queryModel.showCount", "5000"),
    ("queryModel.currentPage", "1"),
    ("queryModel.sortName", ""),
    ("queryModel.sortOrder", "asc"),
    ("time", "1"),
];
```

**After:**
```rust
let form = [
    ("xh_id", xh_id),
    ("xnm", xnm),
    ("xqm", xqm),
    ("_search", "false"),
    ("nd", &timestamp),
    ("queryModel.showCount", "5000"),
    ("queryModel.currentPage", "1"),
    ("queryModel.sortName", " "),  // Changed from "" to " " (space)
    ("queryModel.sortOrder", "asc"),
    ("time", "0"),  // Changed from "1" to "0"
];
```

**Key Changes:**
- Added `xh_id` field
- `xnm` and `xqm` now use parameters instead of hardcoded empty strings
- `queryModel.sortName` changed from empty string to single space
- `time` changed from "1" to "0"

### 4. Cookie Changes
**Before:**
```rust
cookies.insert("__access_token".into(), access_token.into());
cookies.insert("JSESSIONID".into(), jwglxt_jsession.into());
cookies.insert("route".into(), route.into());
```

**After:**
```rust
cookies.insert("JSESSIONID".into(), jwglxt_jsession.into());
cookies.insert("route".into(), route.into());
```

**Key Changes:**
- Removed `__access_token` cookie (not used in Python version)

### 5. Handler Updates (`handlers.rs`)

Added `RawScoresParams` struct in `api_types.rs`:
```rust
#[derive(Debug, Deserialize)]
pub struct RawScoresParams {
    pub xh_id: Option<String>, // 学号ID
    pub xnm: Option<String>,   // 学年码
    pub xqm: Option<String>,   // 学期码
}
```

Updated `get_raw_scores` handler to accept query parameters:
```rust
pub async fn get_raw_scores(
    Extension(student_id): Extension<String>,
    Query(params): Query<crate::api_types::RawScoresParams>,
) -> Result<Json<ApiResponse<Vec<Score>>>, (StatusCode, Json<ApiResponse<()>>)>
```

Handler now extracts and passes parameters:
```rust
let xh_id = params.xh_id.as_deref().unwrap_or(&student_id);
let xnm = params.xnm.as_deref().unwrap_or("");
let xqm = params.xqm.as_deref().unwrap_or("");

match tyust_get_raw_scores(
    &auth_cache.jwglxt_jsession,
    &auth_cache.route,
    xh_id,
    xnm,
    xqm,
)
```

### 6. Test Function Updates
Updated `test_get_user_raw_scores` to use new function signature:
```rust
let score_items = tyust_get_raw_scores(
    &jwglxt_jsession,
    &route,
    student_id, // xh_id
    "",         // xnm (empty for all years)
    "",         // xqm (empty for all semesters)
)
.await?;
```

## API Usage

### Frontend Usage Example
```javascript
// Get all scores
fetch('/api/raw_scores')

// Get scores for specific academic year
fetch('/api/raw_scores?xnm=2023')

// Get scores for specific semester
fetch('/api/raw_scores?xnm=2023&xqm=3')

// Get scores for specific student (if different from logged-in user)
fetch('/api/raw_scores?xh_id=202112181110')
```

## Benefits
1. **Flexibility**: Supports filtering scores by academic year and semester
2. **Python Compatibility**: Matches the Python implementation exactly
3. **Simplified Authentication**: Removed unused access_token parameter
4. **Better API Design**: Query parameters allow for more granular data retrieval

## Handling Different Response Structures

The new endpoint (`cxDgXscj`) returns a different data structure than the old endpoint (`cxXsYscj`). Many fields that were present in the old response are missing in the new one.

### Solution: Optional Fields
To handle this, we've made many fields in the `ScoreItem` struct optional using `#[serde(default)]`:

**Fields made optional:**
- `bfzcj`, `bh`, `bh_id`, `bj` - Basic info fields
- `cjbdczr`, `cjbdsj`, `cjsfzf` - Grade change tracking fields
- `date`, `dateDigit`, `dateDigitSeparator`, `day`, `month`, `year` - Date fields
- `jg_id`, `jgpxzd`, `jxb_id`, `jxbmc`, `kcbj` - Organization and class fields
- `khfsmc`, `kkbmmc` - Assessment and department fields
- `ksxz`, `ksxzdm` - Exam nature fields
- `key`, `listnav`, `localeKey` - UI/navigation fields
- `njdm_id`, `njmc` - Grade level fields
- `pageTotal`, `pageable`, `rangeable`, `row_id` - Pagination fields
- `rwzxs`, `sfdkbcx`, `sfxwkc`, `sfzh`, `sfzx` - Various flag fields
- `tjrxm`, `tjsj` - Submission info fields
- `totalResult`, `userModel`, `queryModel` - Model fields
- `xb`, `xbm`, `xz`, `zxs` - Student info fields

**Core fields (always present):**
- `cj` - Score (grade)
- `jd` - Grade point
- `jgmc` - Institution name
- `jsxm` - Teacher name
- `kch`, `kch_id` - Course number
- `kclbmc` - Course category
- `kcmc` - Course name
- `kcxzmc` - Course nature
- `xf` - Credits
- `xfjd` - Credit grade point
- `xh`, `xh_id` - Student number
- `xm` - Student name
- `xnm`, `xnmmc` - Academic year
- `xqm`, `xqmmc` - Semester
- `zyh_id`, `zymc` - Major info

### Default Implementations
Added `Default` trait to `QueryModel` and `UserModel` structs to support the `#[serde(default)]` attribute on nested fields.

## Compatibility Notes
- The endpoint change (`cxXsYscj` -> `cxDgXscj`) returns different data structures with many optional fields
- Old callers of `tyust_get_raw_scores` need to be updated to pass the new parameters
- All query parameters are optional and default to sensible values (student_id for xh_id, empty strings for xnm/xqm)
- Missing fields in the API response will be filled with default values (empty strings for String, false for bool, 0 for i32)