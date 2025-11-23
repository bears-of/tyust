import {
  loginRequest
} from "../../api/main"

Page({
  data: {
    stuId: '', // 学号
    password: '', // 密码
    saveCount: true, // 是否记住账号
    isPasswordVisible: false, // 核心修改：控制密码显示/隐藏
  },

  onLoad(options) {
    this.initAccount()
  },

  // 初始化账号
  initAccount() {
    const accountCache = wx.getStorageSync("account")
    if (accountCache) {
      this.setData({
        ...accountCache
      })
    }
  },

  // 核心修改：切换密码显示状态
  togglePassword() {
    this.setData({
      isPasswordVisible: !this.data.isPasswordVisible
    })
  },

  // 记住密码复选框切换
  switchStatus() {
    this.setData({
      saveCount: !this.data.saveCount
    })
  },

  // 登录
  login() {
    const that = this
    // 简单的非空校验
    if (!that.data.stuId || !that.data.password) {
      wx.showToast({
        title: '请输入账号和密码',
        icon: 'none'
      })
      return;
    }

    const postData = {
      stuId: that.data.stuId,
      password: that.data.password
    }
    
    wx.showLoading({
      title: '登录中...',
      mask: true
    })

    loginRequest(postData).then(res => {
      wx.hideLoading()
      if (res.code == -1) {
        wx.showToast({
          title: res.msg,
          icon: 'none'
        })
        return
      }

      console.log(res.data)
      
      // 登录成功逻辑
      wx.setStorageSync('token', res.data.token)
      wx.setStorageSync('studentId', res.data.studentId)
      wx.setStorageSync('name', res.data.name)
      wx.setStorageSync('class', res.data.class || '未知班级')
      
      // 处理记住密码
      if (that.data.saveCount) {
        wx.setStorageSync('account', postData)
      } else {
        wx.removeStorageSync('account')
      }

      wx.showToast({
        title: '登录成功',
        icon: 'success'
      })
      
      setTimeout(() => {
        wx.switchTab({
          url: '/pages/index/index',
        })
      }, 1500);
    })
  }
})