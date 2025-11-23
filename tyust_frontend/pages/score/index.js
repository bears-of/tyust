import {
  getScoreListRequest,
  getRawScoreListRequest
} from '../../api/main'
const scoreCacheKey = "scores"
const rawScoreCacheKey = "rawScores"
Page({

  /**
   * 页面的初始数据
   */
  data: {
    type: 1, // 1为有效成绩，2为原始成绩
    list: [], // 成绩列表
    termIndex: 0, // 当前学期索引
    isUpdating: false, // <-- 新增：刷新状态
  },

  /**
   * 生命周期函数--监听页面加载
   */
  onLoad(options) {
    this.getList()
  },

  getList() {
    const cache = wx.getStorageSync(this.data.type == 1 ? scoreCacheKey : rawScoreCacheKey)
    if (cache) {
      this.setData({
        list: cache
      })
      return
    }
    this.update()
  },

  update() {
    const that = this
    // 1. 开始刷新时，设置状态为 true
    that.setData({
      isUpdating: true
    }) 
    
    let p = null
    if (that.data.type == 1) {
      p = getScoreListRequest()
    } else {
      p = getRawScoreListRequest()
    }
    
    p.then(res => {
      that.setData({
        list: res.data
      })
      wx.setStorageSync(that.data.type == 1 ? scoreCacheKey : rawScoreCacheKey, res.data)
    }).finally(() => { // 无论成功失败，最后都要关闭加载状态
      // 2. 刷新结束时，设置状态为 false
      that.setData({
        isUpdating: false
      }) 
      wx.showToast({
        title: '成绩已更新',
        icon: 'success',
        duration: 1000
      })
    })
  },

  // 切换成绩类型
  changeScoreType(e) {
    const type = e.currentTarget.dataset.type
    this.setData({
      type
    })
    this.getList()
  },

  changeTerm(e) {
    const termIndex = e.detail.value
    this.setData({
      termIndex
    })
  }
})