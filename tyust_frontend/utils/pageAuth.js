/**
 * 页面认证混入
 * 在页面 onLoad 时自动检查登录状态
 *
 * 使用方法：
 * import pageAuth from '../../utils/pageAuth'
 *
 * Page({
 *   mixins: [pageAuth], // 如果框架支持mixins
 *   // 或者手动调用
 *   onLoad(options) {
 *     pageAuth.checkAuth()
 *   }
 * })
 */

const auth = require('./auth')

/**
 * 检查页面认证状态
 * @param {boolean} showToast 是否显示提示
 * @returns {boolean} 是否已登录
 */
function checkAuth(showToast = true) {
  if (!auth.hasToken()) {
    if (showToast) {
      wx.showToast({
        title: '请先登录',
        icon: 'none',
        duration: 1500
      })
    }

    setTimeout(() => {
      wx.reLaunch({
        url: '/pages/login/index'
      })
    }, showToast ? 1500 : 0)

    return false
  }
  return true
}

/**
 * 页面混入对象
 * 可以在小程序页面中使用（如果支持）
 */
const pageAuthMixin = {
  onLoad(options) {
    // 检查登录状态
    this._checkPageAuth()
  },

  onShow() {
    // 页面显示时也检查一次
    if (this.data._authChecked) {
      this._checkPageAuth(false)
    }
  },

  _checkPageAuth(showToast = true) {
    const isLoggedIn = checkAuth(showToast)
    this.setData({
      _authChecked: true,
      _isLoggedIn: isLoggedIn
    })
    return isLoggedIn
  }
}

/**
 * 装饰器方式（高阶函数）
 * 包装页面配置对象，自动添加认证检查
 *
 * @param {Object} pageConfig 页面配置对象
 * @param {Object} options 选项
 * @param {boolean} options.checkOnLoad 是否在onLoad时检查
 * @param {boolean} options.checkOnShow 是否在onShow时检查
 * @returns {Object} 包装后的页面配置
 */
function withAuth(pageConfig, options = {}) {
  const { checkOnLoad = true, checkOnShow = false } = options

  const originalOnLoad = pageConfig.onLoad
  const originalOnShow = pageConfig.onShow

  if (checkOnLoad) {
    pageConfig.onLoad = function(loadOptions) {
      // 先检查认证
      if (!checkAuth(true)) {
        return
      }

      // 调用原始的onLoad
      if (originalOnLoad) {
        originalOnLoad.call(this, loadOptions)
      }
    }
  }

  if (checkOnShow) {
    pageConfig.onShow = function() {
      // 检查认证（不显示toast）
      checkAuth(false)

      // 调用原始的onShow
      if (originalOnShow) {
        originalOnShow.call(this)
      }
    }
  }

  return pageConfig
}

module.exports = {
  checkAuth,
  pageAuthMixin,
  withAuth
}
