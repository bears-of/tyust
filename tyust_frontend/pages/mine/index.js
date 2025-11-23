// pages/mine/index.js
import { updateAvatarRequest } from '../../api/main'

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
    const app = getApp()
    const token = wx.getStorageSync('token')
    const studentId = wx.getStorageSync('studentId')
    const name = wx.getStorageSync('name')
    const classInfo = wx.getStorageSync('class')
    let avatarUrl = wx.getStorageSync('avatarUrl')
    
    // 如果avatarUrl是相对路径，则添加后端基础URL
    if (avatarUrl && avatarUrl.startsWith('/static/')) {
      const baseUrl = app.getConfig("baseUrl")
      // 从baseUrl中提取基础URL (去掉/api部分)
      const backendBaseUrl = baseUrl.replace('/api', '')
      avatarUrl = backendBaseUrl + avatarUrl
    }

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
    
    // 读取文件并转换为base64
    wx.getFileSystemManager().readFile({
      filePath: avatarUrl,
      encoding: 'base64',
      success: (res) => {
        // 上传头像到后端
        this.uploadAvatarToServer(`data:image/jpeg;base64,${res.data}`)
      },
      fail: (err) => {
        console.error('读取头像文件失败:', err)
        wx.showToast({
          title: '读取头像失败',
          icon: 'none'
        })
      }
    })
  },

  /**
   * 上传头像到服务器
   */
  uploadAvatarToServer(base64Data) {
    const updateAvatarRequest = require("../../api/main").updateAvatarRequest
    const app = getApp()
    
    updateAvatarRequest({ avatarData: base64Data })
      .then(res => {
        if (res.code === 0) {
          // 保存头像URL到本地存储
          wx.setStorageSync('avatarUrl', res.data.avatarUrl)
          
          // 如果avatarUrl是相对路径，则添加后端基础URL用于显示
          let displayAvatarUrl = res.data.avatarUrl
          if (displayAvatarUrl && displayAvatarUrl.startsWith('/static/')) {
            const baseUrl = app.getConfig("baseUrl")
            // 从baseUrl中提取基础URL (去掉/api部分)
            const backendBaseUrl = baseUrl.replace('/api', '')
            displayAvatarUrl = backendBaseUrl + displayAvatarUrl
          }
          
          // 更新页面显示
          this.setData({
            'userInfo.avatarUrl': displayAvatarUrl
          })
          
          wx.showToast({
            title: '头像已更新',
            icon: 'success'
          })
        } else {
          wx.showToast({
            title: '头像更新失败',
            icon: 'none'
          })
        }
      })
      .catch(err => {
        console.error('上传头像失败:', err)
        wx.showToast({
          title: '网络错误',
          icon: 'none'
        })
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