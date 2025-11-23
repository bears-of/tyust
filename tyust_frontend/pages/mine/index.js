// pages/mine/index.js
Page({

  /**
   * 页面的初始数据
   */
  data: {
    userInfo: {},
    isLogin: false,
    showAboutDialog: false
  },

  /**
   * 生命周期函数--监听页面加载
   */
  onLoad(options) {
    this.loadUserInfo()
  },

  /**
   * 生命周期函数--监听页面显示
   */
  onShow() {
    this.loadUserInfo()
  },

  /**
   * 加载用户信息
   */
  loadUserInfo() {
    const token = wx.getStorageSync('token')
    const studentId = wx.getStorageSync('studentId')
    const name = wx.getStorageSync('name')
    const classInfo = wx.getStorageSync('class')
    const avatarUrl = wx.getStorageSync('avatarUrl')

    if (token && studentId) {
      this.setData({
        isLogin: true,
        userInfo: {
          studentId,
          name: name || '学生',
          class: classInfo || '未知班级',
          avatarUrl: avatarUrl || ''
        }
      })
    } else {
      this.setData({
        isLogin: false,
        userInfo: {}
      })
    }
  },

  /**
   * 选择头像
   */
  onChooseAvatar(e) {
    const { avatarUrl } = e.detail
    
    // 保存头像到本地存储
    wx.setStorageSync('avatarUrl', avatarUrl)
    
    // 更新页面显示
    this.setData({
      'userInfo.avatarUrl': avatarUrl
    })

    wx.showToast({
      title: '头像已更新',
      icon: 'success'
    })
  },

  /**
   * 显示关于弹窗
   */
  showAbout() {
    this.setData({
      showAboutDialog: true
    })
  },

  /**
   * 关闭关于弹窗
   */
  closeAbout() {
    this.setData({
      showAboutDialog: false
    })
  },

  /**
   * 退出登录
   */
  logout() {
    wx.showModal({
      title: '提示',
      content: '确定要退出登录吗？',
      success: (res) => {
        if (res.confirm) {
          // 清除本地存储
          wx.clearStorageSync()
          
          // 更新页面状态
          this.setData({
            isLogin: false,
            userInfo: {}
          })

          // 提示成功
          wx.showToast({
            title: '已退出登录',
            icon: 'success'
          })

          // 跳转到登录页
          setTimeout(() => {
            wx.reLaunch({
              url: '/pages/login/index'
            })
          }, 1500)
        }
      }
    })
  }
})