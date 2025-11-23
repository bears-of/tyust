// index.js - 保持大部分逻辑不变，这里不需要大改
const app = getApp()
import { getNowWeek } from '../../utils/util'

Page({
  data: {
    navList: [
      { title: '查课表', icon: '/asset/imgs/course.png', path: '/pages/course/index' },
      { title: '查成绩', icon: '/asset/imgs/score.png', path: '/pages/score/index' },
      { title: '查考勤', icon: '/asset/imgs/attendance.png', path: '/pages/attendance/index' },
      { title: '校历', icon: '/asset/imgs/calendar.png', path: '/pages/calendar/index' },
    ],
    startDate: '2023/02/20', 
    totalWeek: 20,
    todayCourseList: [],
    todayWeek: 1,
    todayWeeks: 1
  },

  onLoad() {
    // 可以在这里加个加载动画，显得更高级
    wx.showLoading({ title: '加载中...', mask: true })
    
    // 模拟延迟，或者直接加载
    this.getTodayCourseList();
    
    setTimeout(() => {
        wx.hideLoading();
    }, 300);
  },

  nav(e) {
    const index = e.currentTarget.dataset.index
    const path = this.data.navList[index].path
    // 增加点击震动反馈，提升交互手感
    wx.vibrateShort();
    
    wx.navigateTo({
      url: path,
      fail() {
        wx.switchTab({ url: path })
      }
    })
  },

  getTodayCourseList() {
    const todayWeek = new Date().getDay()
    const todayWeeks = getNowWeek(this.data.startDate, this.data.totalWeek)
    const courseList = wx.getStorageSync('courses') || [] // 防止空数组报错
    
    const todayCourseList = courseList.filter(item => {
      // 兼容处理：如果 item.weeks 是字符串需要注意转换，这里假设是数组
      return item.week == todayWeek && (item.weeks && item.weeks.indexOf(todayWeeks) > -1)
    })
    
    todayCourseList.sort((a, b) => {
      return a.section - b.section
    })

    this.setData({
      todayWeek: todayWeek === 0 ? 7 : todayWeek, // 处理周日为0的情况
      todayWeeks,
      todayCourseList
    })
  }
})