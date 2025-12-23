import { loginRequest } from "../../api/main";
const auth = require("../../utils/auth");

Page({
  data: {
    stuId: "", // 学号
    password: "", // 密码
    saveCount: true, // 是否记住账号
    isPasswordVisible: false, // 控制密码显示/隐藏
  },

  onLoad(options) {
    this.initAccount();
    this.checkLoginStatus();
  },

  // 检查登录状态
  checkLoginStatus() {
    // 如果已经登录，直接跳转到首页
    if (auth.hasToken()) {
      wx.showToast({
        title: "您已登录",
        icon: "success",
        duration: 1500,
      });
      setTimeout(() => {
        wx.switchTab({
          url: "/pages/index/index",
        });
      }, 1500);
    }
  },

  // 初始化账号
  initAccount() {
    const accountCache = wx.getStorageSync(auth.STORAGE_KEYS.ACCOUNT);
    if (accountCache) {
      this.setData({
        stuId: accountCache.stuId || "",
        password: accountCache.password || "",
        saveCount: true,
      });
    }
  },

  // 切换密码显示状态
  togglePassword() {
    this.setData({
      isPasswordVisible: !this.data.isPasswordVisible,
    });
  },

  // 记住密码复选框切换
  switchStatus() {
    this.setData({
      saveCount: !this.data.saveCount,
    });
  },

  // 登录
  login() {
    const that = this;

    // 简单的非空校验
    if (!that.data.stuId || !that.data.password) {
      wx.showToast({
        title: "请输入账号和密码",
        icon: "none",
      });
      return;
    }

    const postData = {
      stuId: that.data.stuId,
      password: that.data.password,
    };

    wx.showLoading({
      title: "登录中...",
      mask: true,
    });

    loginRequest(postData)
      .then((res) => {
        wx.hideLoading();

        if (res.code == -1) {
          wx.showToast({
            title: res.msg,
            icon: "none",
          });
          return;
        }

        console.log("登录成功:", res.data);

        // 使用auth工具保存token和用户信息
        auth.setToken(res.data.token);
        auth.setUserInfo({
          studentId: res.data.studentId,
          name: res.data.name,
          class: res.data.class || "未知班级",
          avatarUrl: res.data.avatarUrl || "",
        });

        // 获取微信用户信息（包括头像）- 仅当数据库中没有头像时
        if (!res.data.avatarUrl) {
          that.getWechatUserInfo();
        }

        // 处理记住密码
        if (that.data.saveCount) {
          wx.setStorageSync(auth.STORAGE_KEYS.ACCOUNT, postData);
        } else {
          wx.removeStorageSync(auth.STORAGE_KEYS.ACCOUNT);
        }

        // 立即获取课程表数据
        that.fetchCourseData(() => {
          wx.showToast({
            title: "登录成功",
            icon: "success",
          });

          setTimeout(() => {
            wx.switchTab({
              url: "/pages/index/index",
            });
          }, 1500);
        });
      })
      .catch((err) => {
        wx.hideLoading();
        console.error("登录失败:", err);
        wx.showToast({
          title: err.msg || "登录失败，请重试",
          icon: "none",
        });
      });
  },

  // 获取微信用户信息
  getWechatUserInfo() {
    const that = this;
    wx.getUserInfo({
      success: function (userRes) {
        // 保存头像URL到本地存储
        const avatarUrl = userRes.userInfo.avatarUrl;
        wx.setStorageSync(auth.STORAGE_KEYS.AVATAR_URL, avatarUrl);

        // 上传头像到服务器
        that.uploadAvatarToServer(avatarUrl);
      },
      fail: function () {
        console.log("获取微信用户信息失败");
      },
    });
  },

  // 上传头像到服务器
  uploadAvatarToServer(avatarUrl) {
    const updateAvatarRequest = require("../../api/main").updateAvatarRequest;

    updateAvatarRequest({ avatarUrl: avatarUrl })
      .then((res) => {
        if (res.code === 0) {
          console.log("头像已更新到服务器");
        } else {
          console.log("头像更新失败:", res.msg);
        }
      })
      .catch((err) => {
        console.error("上传头像失败:", err);
      });
  },

  // 立即获取课程表数据
  fetchCourseData(callback) {
    const that = this;
    const getCourseListRequest = require("../../api/main").getCourseListRequest;
    const getSemesterConfigRequest =
      require("../../api/main").getSemesterConfigRequest;

    // 先获取学期配置
    getSemesterConfigRequest()
      .then((configRes) => {
        if (configRes.code === 0 && configRes.data) {
          // 保存学期配置到本地存储
          wx.setStorageSync(auth.STORAGE_KEYS.SEMESTER_CONFIG, configRes.data);

          // 然后获取课程数据
          getCourseListRequest()
            .then((courseRes) => {
              if (courseRes.code === 0) {
                // 保存课程数据到本地存储
                wx.setStorageSync(auth.STORAGE_KEYS.COURSES, courseRes.data);
                console.log("课程数据获取成功");
              } else {
                console.log("获取课程数据失败:", courseRes.msg);
              }
              // 执行回调函数
              if (callback) callback();
            })
            .catch((err) => {
              console.error("获取课程数据异常:", err);
              // 即使获取失败也执行回调函数
              if (callback) callback();
            });
        } else {
          console.log("获取学期配置失败:", configRes.msg);
          // 即使获取失败也执行回调函数
          if (callback) callback();
        }
      })
      .catch((err) => {
        console.error("获取学期配置异常:", err);
        // 即使获取失败也执行回调函数
        if (callback) callback();
      });
  },
});
