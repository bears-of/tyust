Page({

  /**
   * 页面的初始数据
   */
  data: {
    
  },

  /**
   * 生命周期函数--监听页面加载
   */
  onLoad(options) {
    let info = options.info || ''
    if (info == '') {
      wx.showToast({
        title: '页面不存在',
        icon: 'none'
      })
      setTimeout(() => {
        wx.navigateBack({
          delta: 1,
        })
      }, 1500);
      return
    }
    
    try {
      info = JSON.parse(info)
      // 如果你需要处理节次显示的文字，可以在这里做
      // 比如： info.rawSectionText = '第' + info.rawSection + '节'
      this.setData({
        info
      })
    } catch (e) {
      console.error("解析课程数据失败", e)
    }
  },
})