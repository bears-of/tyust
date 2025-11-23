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
    todayWeeks: 1,
    userName: '同学' // 默认值
  },

  onLoad() {
    // 可以在这里加个加载动画，显得更高级
    wx.showLoading({ title: '加载中...', mask: true })
    
    // 获取用户姓名
    const userName = wx.getStorageSync('name') || '同学'
    this.setData({
      userName: userName
    })
    
    // 立即刷新今日课程数据
    this.refreshTodayCourseList();
    
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

  // 刷新今日课程数据
  refreshTodayCourseList() {
    const that = this;
    
    // 引入课程数据请求函数
    const getCourseListRequest = require('../../api/main').getCourseListRequest;
    const getSemesterConfigRequest = require('../../api/main').getSemesterConfigRequest;
    
    // 先获取最新的学期配置
    getSemesterConfigRequest().then(configRes => {
      if (configRes.code === 0 && configRes.data) {
        // 更新本地存储的学期配置
        wx.setStorageSync('semesterConfig', configRes.data);
        
        // 获取最新的课程数据
        getCourseListRequest().then(courseRes => {
          if (courseRes.code === 0) {
            // 更新本地存储的课程数据
            wx.setStorageSync('courses', courseRes.data);
            
            // 更新今日课程列表
            that.getTodayCourseList();
          } else {
            // 如果获取失败，仍然使用本地缓存的数据
            that.getTodayCourseList();
          }
        }).catch(err => {
          console.error('获取课程数据失败:', err);
          // 如果请求失败，仍然使用本地缓存的数据
          that.getTodayCourseList();
        });
      } else {
        // 如果获取学期配置失败，仍然使用本地缓存的数据
        that.getTodayCourseList();
      }
    }).catch(err => {
      console.error('获取学期配置失败:', err);
      // 如果请求失败，仍然使用本地缓存的数据
      that.getTodayCourseList();
    });
  },

  getTodayCourseList() {
    // 从本地存储获取学期配置
    const semesterConfig = wx.getStorageSync('semesterConfig')
    let startDate = this.data.startDate
    let totalWeek = this.data.totalWeek
    
    if (semesterConfig && semesterConfig.semester_start_date) {
      // 将 YYYY-MM-DD 格式转换为 YYYY/MM/DD 格式
      startDate = semesterConfig.semester_start_date.replace(/-/g, '/')
      totalWeek = 20 // 默认20周
    }
    
    const todayWeek = new Date().getDay() // 周日为0，周一为1，...，周六为6
    const todayWeeks = getNowWeek(startDate, totalWeek)
    const courseList = wx.getStorageSync('courses') || [] // 防止空数组报错
    
    const todayCourseList = courseList.filter(item => {
      // 兼容处理：如果 item.weeks 是字符串需要注意转换，这里假设是数组
      // 确保今天的课程在当前周次内
      return item.week == (todayWeek === 0 ? 7 : todayWeek) && (item.weeks && item.weeks.includes(todayWeeks))
    })
    
    todayCourseList.sort((a, b) => {
      return a.section - b.section
    })

    this.setData({
      todayWeek,
      todayWeeks,
      todayCourseList
    })
  }
})