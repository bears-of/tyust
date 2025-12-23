// pages/mine/index.js
import { updateAvatarRequest } from "../../api/main";
const auth = require("../../utils/auth");
const pageAuth = require("../../utils/pageAuth");

Page({
  /**
   * 页面的初始数据
   */
  data: {
    userInfo: {},
    isLogin: false,
    showAboutDialog: false,
  },

  /**
   * 生命周期函数--监听页面加载
   */
  onLoad(options) {
    // 检查登录状态（不强制跳转，因为mine页面可以展示未登录状态）
    this.checkLoginStatus();
    this.loadUserInfo();
  },

  /**
   * 生命周期函数--监听页面显示
   */
  onShow() {
    this.checkLoginStatus();
    this.loadUserInfo();
  },

  /**
   * 检查登录状态
   */
  checkLoginStatus() {
    const isLogin = auth.hasToken();
    this.setData({
      isLogin,
    });
  },

  /**
   * 加载用户信息
   */
  loadUserInfo() {
    const app = getApp();

    if (!auth.hasToken()) {
      this.setData({
        isLogin: false,
        userInfo: {},
      });
      return;
    }

    // 使用auth工具获取用户信息
    const userInfo = auth.getUserInfo();
    let avatarUrl = userInfo.avatarUrl;

    // 如果avatarUrl是相对路径，则添加后端基础URL
    if (avatarUrl && avatarUrl.startsWith("/static/")) {
      const baseUrl = app.getConfig("baseUrl");
      // 从baseUrl中提取基础URL (去掉/api部分)
      const backendBaseUrl = baseUrl.replace("/api", "");
      avatarUrl = backendBaseUrl + avatarUrl;
    }

    this.setData({
      isLogin: true,
      userInfo: {
        studentId: userInfo.studentId,
        name: userInfo.name || "学生",
        class: userInfo.class || "未知班级",
        avatarUrl: avatarUrl || "",
      },
    });
  },

  /**
   * 选择头像
   */
  onChooseAvatar(e) {
    // 检查登录状态
    if (!auth.checkLogin()) {
      return;
    }

    const { avatarUrl } = e.detail;

    // 读取文件并转换为base64
    wx.getFileSystemManager().readFile({
      filePath: avatarUrl,
      encoding: "base64",
      success: (res) => {
        // 上传头像到后端
        this.uploadAvatarToServer(`data:image/jpeg;base64,${res.data}`);
      },
      fail: (err) => {
        console.error("读取头像文件失败:", err);
        wx.showToast({
          title: "读取头像失败",
          icon: "none",
        });
      },
    });
  },

  /**
   * 上传头像到服务器
   */
  uploadAvatarToServer(base64Data) {
    const app = getApp();

    wx.showLoading({
      title: "上传中...",
      mask: true,
    });

    updateAvatarRequest({ avatarData: base64Data })
      .then((res) => {
        wx.hideLoading();

        if (res.code === 0) {
          // 保存头像URL到本地存储
          wx.setStorageSync(auth.STORAGE_KEYS.AVATAR_URL, res.data.avatarUrl);

          // 如果avatarUrl是相对路径，则添加后端基础URL用于显示
          let displayAvatarUrl = res.data.avatarUrl;
          if (displayAvatarUrl && displayAvatarUrl.startsWith("/static/")) {
            const baseUrl = app.getConfig("baseUrl");
            // 从baseUrl中提取基础URL (去掉/api部分)
            const backendBaseUrl = baseUrl.replace("/api", "");
            displayAvatarUrl = backendBaseUrl + displayAvatarUrl;
          }

          // 更新页面显示
          this.setData({
            "userInfo.avatarUrl": displayAvatarUrl,
          });

          wx.showToast({
            title: "头像已更新",
            icon: "success",
          });
        } else {
          wx.showToast({
            title: res.msg || "头像更新失败",
            icon: "none",
          });
        }
      })
      .catch((err) => {
        wx.hideLoading();
        console.error("上传头像失败:", err);
        wx.showToast({
          title: "网络错误",
          icon: "none",
        });
      });
  },

  /**
   * 跳转到登录页
   */
  goToLogin() {
    wx.navigateTo({
      url: "/pages/login/index",
    });
  },

  /**
   * 显示关于弹窗
   */
  showAbout() {
    this.setData({
      showAboutDialog: true,
    });
  },

  /**
   * 关闭关于弹窗
   */
  closeAbout() {
    this.setData({
      showAboutDialog: false,
    });
  },

  /**
   * 退出登录
   */
  logout() {
    // 检查登录状态
    if (!auth.hasToken()) {
      wx.showToast({
        title: "您还未登录",
        icon: "none",
      });
      return;
    }

    wx.showModal({
      title: "提示",
      content: "确定要退出登录吗？",
      success: (res) => {
        if (res.confirm) {
          // 使用auth工具清除登录信息（保留账号密码）
          auth.clearAuth(true);

          // 更新页面状态
          this.setData({
            isLogin: false,
            userInfo: {},
          });

          // 提示成功
          wx.showToast({
            title: "已退出登录",
            icon: "success",
            duration: 1500,
          });

          // 跳转到登录页
          setTimeout(() => {
            wx.reLaunch({
              url: "/pages/login/index",
            });
          }, 1500);
        }
      },
    });
  },
});
