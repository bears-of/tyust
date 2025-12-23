import { getScoreListRequest, getRawScoreListRequest } from "../../api/main";
const auth = require("../../utils/auth");
const pageAuth = require("../../utils/pageAuth");
const scoreCacheKey = "scores";
const rawScoreCacheKey = "rawScores";
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
    // 检查登录状态
    if (!pageAuth.checkAuth()) {
      return;
    }
    this.getList();
  },

  getList() {
    const cache = wx.getStorageSync(
      this.data.type == 1 ? scoreCacheKey : rawScoreCacheKey,
    );
    if (cache) {
      this.setData({
        list: cache,
      });
      return;
    }
    this.update();
  },

  /**
   * 转换后端返回的成绩数据为前端需要的格式
   * 后端返回: [{ semester, course, score, gpa, credit, teacher, courseType }, ...]
   * 前端需要: [{ termName, scoreList: [{ name, score, ... }] }, ...]
   */
  transformScoreData(backendData, isRaw = false) {
    // 按学期分组
    const termMap = new Map();

    backendData.forEach((item) => {
      const termName = item.semester;
      if (!termMap.has(termName)) {
        termMap.set(termName, []);
      }

      // 转换单个成绩项
      const scoreItem = {
        name: item.course,
        score: item.score,
        credit: item.credit,
        gpa: item.gpa,
        teacher: item.teacher,
        courseType: item.courseType,
      };

      // 原始成绩需要额外字段
      if (isRaw) {
        // 尝试解析成绩中的平时、期中、期末成绩
        // 如果成绩是数字，则作为综合成绩
        const numScore = parseFloat(item.score);
        if (!isNaN(numScore)) {
          scoreItem.complexScore = item.score;
        } else {
          // 如果成绩不是数字（如"良"、"优"），也作为综合成绩
          scoreItem.complexScore = item.score;
        }

        // 这里可以根据实际情况添加平时、期中、期末成绩的解析逻辑
        // 目前后端返回的是总成绩，如果需要详细成绩需要后端提供更多数据
        scoreItem.normalScore = "";
        scoreItem.midtermScore = "";
        scoreItem.finalScore = "";
      }

      termMap.get(termName).push(scoreItem);
    });

    // 转换为数组格式
    const result = [];
    termMap.forEach((scoreList, termName) => {
      result.push({
        termName,
        scoreList,
      });
    });

    // 按学期排序（最新的在前面）
    result.sort((a, b) => {
      // 简单的字符串比较，可以根据需要改进
      return b.termName.localeCompare(a.termName);
    });

    return result;
  },

  update() {
    const that = this;

    // 检查登录状态
    if (!auth.checkLogin()) {
      return;
    }

    // 1. 开始刷新时，设置状态为 true
    that.setData({
      isUpdating: true,
    });

    let p = null;
    if (that.data.type == 1) {
      p = getScoreListRequest();
    } else {
      p = getRawScoreListRequest();
    }

    p.then((res) => {
      // 转换后端数据格式
      const transformedData = that.transformScoreData(
        res.data,
        that.data.type === 2,
      );

      that.setData({
        list: transformedData,
      });
      wx.setStorageSync(
        that.data.type == 1 ? scoreCacheKey : rawScoreCacheKey,
        transformedData,
      );
    })
      .catch((err) => {
        console.error("获取成绩失败:", err);
        wx.showToast({
          title: "获取成绩失败",
          icon: "error",
          duration: 2000,
        });
      })
      .finally(() => {
        // 无论成功失败，最后都要关闭加载状态
        // 2. 刷新结束时，设置状态为 false
        that.setData({
          isUpdating: false,
        });
        wx.showToast({
          title: "成绩已更新",
          icon: "success",
          duration: 1000,
        });
      });
  },

  // 切换成绩类型
  changeScoreType(e) {
    const type = e.currentTarget.dataset.type;
    this.setData({
      type,
      termIndex: 0, // 切换类型时重置学期索引
    });
    this.getList();
  },

  changeTerm(e) {
    const termIndex = e.detail.value;
    this.setData({
      termIndex,
    });
  },
});
