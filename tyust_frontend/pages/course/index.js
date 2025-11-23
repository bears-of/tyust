import {
  getCourseListRequest,
  getSemesterConfigRequest
} from '../../api/main'
import {
  getNowWeek
} from '../../utils/util'
const courseCacheKey = "courses"
const courseColorCacheKey = "courseColor"
Page({

  /**
   * 页面的初始数据
   */
  data: {
    nowWeek: 1, // 当前周
    totalWeek: 20, // 周总数
    showSwitchWeek: false, // 显示选择周数弹窗
    weekDayCount: 7,
    startDate: '2023/02/20', // 开学日期
    weekIndexText: ['一', '二', '三', '四', '五', '六', '日'],
    nowMonth: 1, // 当前周的月份
    courseList: [],
    colorList: [
      "#FF9C9C", // 柔和粉
      "#87CEFA", // 天空蓝
      "#90EE90", // 浅绿
      "#DDA0DD", // 梅红
      "#F0E68C", // 卡其
      "#FFB6C1", // 浅粉
      "#20B2AA", // 浅海洋绿
      "#87CEEB", // 更加柔和的蓝
      "#778899", // 冷灰蓝
      "#B0C4DE", // 钢蓝
      "#DB7093", // 苍白紫罗兰红
      "#66CDAA", // 中绿宝石
    ],
    courseColor: {},
    weekCalendar: [1, 2, 3, 4, 5, 6, 7],
    firstEntry: true
  },

  /**
   * 生命周期函数--监听页面加载
   */
  onLoad(options) {
    const {
      windowWidth
    } = wx.getSystemInfoSync()
    this.setData({
      windowWidth
    })
    this.getSemesterConfig()
    this.getWeekDates()
    this.getTodayDate()
  },

  selectWeek() {
    this.setData({
      showSwitchWeek: true
    })
  },

  switchWeek(e) {
    const week = e.currentTarget.dataset.week
    this.setData({
      showSwitchWeek: false
    })
    this.switchWeekFn(week)
  },

  // 切换周数
  switchWeekFn(week) {
    this.setData({
      nowWeek: week
    })
    this.getWeekDates()
  },

  hideSwitchWeek() {
    this.setData({
      showSwitchWeek: false
    })
  },

  getWeekDates() {
    const startDate = new Date(this.data.startDate)
    const addTime = (this.data.nowWeek - 1) * 7 * 24 * 60 * 60 * 1000
    const firstDate = startDate.getTime() + addTime
    const {
      month: nowMonth
    } = this.getDateObject(new Date(firstDate))
    const weekCalendar = []
    for (let i = 0; i < this.data.weekDayCount; i++) {
      const date = new Date(firstDate + i * 24 * 60 * 60 * 1000)
      const {
        day
      } = this.getDateObject(date)
      weekCalendar.push(day)
    }
    this.setData({
      nowMonth,
      weekCalendar
    })
  },

  getDateObject(date = new Date()) {
    const year = date.getFullYear()
    const month = date.getMonth() + 1
    const day = date.getDate()
    return {
      year,
      month,
      day
    }
  },

  // 计算当前周次
  calculateCurrentWeek(startDateStr) {
    const startDate = new Date(startDateStr)
    const currentDate = new Date()
    const timeDiff = currentDate.getTime() - startDate.getTime()
    const daysDiff = Math.ceil(timeDiff / (1000 * 60 * 60 * 24))
    
    if (daysDiff < 0) {
      return 1 // 还未开学，默认第一周
    }
    
    const week = Math.floor(daysDiff / 7) + 1
    return week > this.data.totalWeek ? this.data.totalWeek : week
  },
  
  // 获取当前周次
  getNowWeek() {
    const nowWeek = getNowWeek(this.data.startDate, this.data.totalWeek)
    this.setData({
      nowWeek: nowWeek
    })
  },
  
  // 获取学期配置
  async getSemesterConfig() {
    try {
      const res = await getSemesterConfigRequest()
      if (res.code === 0 && res.data) {
        // 将 YYYY-MM-DD 格式转换为 YYYY/MM/DD 格式
        const startDate = res.data.semester_start_date.replace(/-/g, '/')
        this.setData({
          startDate: startDate
        })
        
        // 计算当前周次
        const currentWeek = this.calculateCurrentWeek(startDate)
        this.setData({
          nowWeek: currentWeek
        })
        
        // 更新周日期
        this.getWeekDates()
        
        // 获取课程数据
        this.getData()
      }
    } catch (error) {
      console.error('获取学期配置失败:', error)
      // 如果获取失败，使用默认值
      this.getData()
    }
  },

  getData() {
    const cache = wx.getStorageSync(courseCacheKey)
    const courseColorCache = wx.getStorageSync(courseColorCacheKey)
    if (cache) {
      this.setData({
        courseList: cache,
      })
      if (!courseColorCache) {
        this.buildCourseColor()
      } else {
        this.setData({
          courseColor: courseColorCache
        })
      }
      return
    }
    this.updateFn(true)
  },

  update() {
    this.updateFn()
  },

  updateFn(firstEntry = false) {
    const that = this
    getCourseListRequest().then(res => {
      that.setData({
        courseList: res.data
      })
      that.buildCourseColor()
      if (!firstEntry) {
        wx.showToast({
          title: '更新成功',
          icon: 'success'
        })
      }
      wx.setStorageSync(courseCacheKey, res.data)
    })
  },

  swiperSwitchWeek(e) {
    if (e.detail.source == '') {
      this.setData({
        firstEntry: false
      })
      return
    }
    const index = e.detail.current
    this.switchWeekFn(index + 1)
  },

  buildCourseColor() {
    const courseColor = {}
    let colorIndex = 0
    this.data.courseList.map(item => {
      if (courseColor[item.name] === undefined) {
        courseColor[item.name] = this.data.colorList[colorIndex]
        colorIndex++
      }
    })
    wx.setStorageSync(courseColorCacheKey, courseColor)
    this.setData({
      courseColor
    })
  },

  // 获取今天日期
  getTodayDate() {
    const {
      month: todayMonth,
      day: todayDay
    } = this.getDateObject()
    this.setData({
      todayMonth,
      todayDay
    })
  },

  navCourseDetail(e) {
    const index = e.currentTarget.dataset.index
    wx.navigateTo({
      url: `/pages/course-detail/index?info=${JSON.stringify(this.data.courseList[index])}`,
    })
  }
})