/**
 * Token 和用户认证管理工具
 */

// 存储键名常量
const STORAGE_KEYS = {
  TOKEN: 'token',
  STUDENT_ID: 'studentId',
  NAME: 'name',
  CLASS: 'class',
  AVATAR_URL: 'avatarUrl',
  ACCOUNT: 'account',
  COURSES: 'courses',
  SEMESTER_CONFIG: 'semesterConfig'
}

/**
 * 检查token是否存在
 * @returns {boolean}
 */
function hasToken() {
  const token = wx.getStorageSync(STORAGE_KEYS.TOKEN)
  return !!token
}

/**
 * 获取token
 * @returns {string}
 */
function getToken() {
  return wx.getStorageSync(STORAGE_KEYS.TOKEN) || ''
}

/**
 * 设置token
 * @param {string} token
 */
function setToken(token) {
  wx.setStorageSync(STORAGE_KEYS.TOKEN, token)
}

/**
 * 移除token
 */
function removeToken() {
  wx.removeStorageSync(STORAGE_KEYS.TOKEN)
}

/**
 * 获取学生ID
 * @returns {string}
 */
function getStudentId() {
  return wx.getStorageSync(STORAGE_KEYS.STUDENT_ID) || ''
}

/**
 * 设置学生ID
 * @param {string} studentId
 */
function setStudentId(studentId) {
  wx.setStorageSync(STORAGE_KEYS.STUDENT_ID, studentId)
}

/**
 * 获取用户信息
 * @returns {object}
 */
function getUserInfo() {
  return {
    studentId: wx.getStorageSync(STORAGE_KEYS.STUDENT_ID) || '',
    name: wx.getStorageSync(STORAGE_KEYS.NAME) || '',
    class: wx.getStorageSync(STORAGE_KEYS.CLASS) || '',
    avatarUrl: wx.getStorageSync(STORAGE_KEYS.AVATAR_URL) || ''
  }
}

/**
 * 设置用户信息
 * @param {object} userInfo
 */
function setUserInfo(userInfo) {
  if (userInfo.studentId) {
    wx.setStorageSync(STORAGE_KEYS.STUDENT_ID, userInfo.studentId)
  }
  if (userInfo.name) {
    wx.setStorageSync(STORAGE_KEYS.NAME, userInfo.name)
  }
  if (userInfo.class) {
    wx.setStorageSync(STORAGE_KEYS.CLASS, userInfo.class)
  }
  if (userInfo.avatarUrl) {
    wx.setStorageSync(STORAGE_KEYS.AVATAR_URL, userInfo.avatarUrl)
  }
}

/**
 * 清除所有认证信息（退出登录）
 * @param {boolean} keepAccount 是否保留账号密码
 */
function clearAuth(keepAccount = true) {
  wx.removeStorageSync(STORAGE_KEYS.TOKEN)
  wx.removeStorageSync(STORAGE_KEYS.STUDENT_ID)
  wx.removeStorageSync(STORAGE_KEYS.NAME)
  wx.removeStorageSync(STORAGE_KEYS.CLASS)
  wx.removeStorageSync(STORAGE_KEYS.COURSES)
  wx.removeStorageSync(STORAGE_KEYS.SEMESTER_CONFIG)

  // 根据参数决定是否保留账号信息
  if (!keepAccount) {
    wx.removeStorageSync(STORAGE_KEYS.ACCOUNT)
    wx.removeStorageSync(STORAGE_KEYS.AVATAR_URL)
  }
}

/**
 * 验证token是否有效（通过发送请求验证）
 * @returns {Promise<boolean>}
 */
function validateToken() {
  return new Promise((resolve) => {
    if (!hasToken()) {
      resolve(false)
      return
    }

    // 这里可以调用一个轻量级的API来验证token
    // 例如获取用户信息接口
    const createRequest = require('./request').default

    createRequest({
      url: '/auth/validate',
      method: 'GET',
      loading: false
    }).then(() => {
      resolve(true)
    }).catch(() => {
      resolve(false)
    })
  })
}

/**
 * 跳转到登录页
 * @param {string} message 提示信息
 */
function redirectToLogin(message = '请先登录') {
  wx.showToast({
    title: message,
    icon: 'none',
    duration: 1500
  })

  setTimeout(() => {
    wx.reLaunch({
      url: '/pages/login/index'
    })
  }, 1500)
}

/**
 * 检查登录状态，未登录则跳转
 * @returns {boolean}
 */
function checkLogin() {
  if (!hasToken()) {
    redirectToLogin('请先登录')
    return false
  }
  return true
}

module.exports = {
  STORAGE_KEYS,
  hasToken,
  getToken,
  setToken,
  removeToken,
  getStudentId,
  setStudentId,
  getUserInfo,
  setUserInfo,
  clearAuth,
  validateToken,
  redirectToLogin,
  checkLogin
}
