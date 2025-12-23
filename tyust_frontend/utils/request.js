const app = getApp();
const auth = require("./auth");

// 标记是否正在处理token失效，避免重复跳转
let isHandlingTokenExpired = false;

export default function createRequest(options) {
  return new Promise((resolve, reject) => {
    const token = auth.getToken();
    const studentId = auth.getStudentId();

    // 如果需要登录但没有token，直接跳转到登录页
    if (options.needLogin !== false && !token) {
      handleTokenExpired("请先登录");
      return reject({ code: 401, msg: "请先登录" });
    }

    const baseUrl = app.getConfig("baseUrl");
    const url = `${baseUrl}${options.url}`;
    const header = {
      token,
      studentId,
    };

    let showLoading = false;
    if (options.loading !== false) {
      showLoading = true;
      wx.showLoading({
        title: "正在加载",
        mask: true,
      });
    }

    wx.request({
      url,
      method: options.method || "GET",
      timeout: options.timeout || 20000,
      header,
      data: options.data || {},
      success(res) {
        res = res.data;
        switch (res.code) {
          // 请求成功
          case 0:
            return resolve(res);

          // 异常信息
          case -1:
            wx.showToast({
              title: res.msg,
              icon: "none",
            });
            reject(res);
            break;

          // 登录已失效，需要重新登录
          case 401:
          case 403:
            handleTokenExpired("登录已失效，请重新登录");
            reject(res);
            break;

          // 其他异常
          default:
            wx.showToast({
              title: res.msg || "服务开小差啦！",
              icon: "none",
            });
            reject(res);
            break;
        }
      },
      fail(err) {
        wx.showToast({
          title: "网络请求失败",
          icon: "none",
        });
        reject(err);
      },
      complete() {
        // 如果有loading，就隐藏
        if (showLoading) {
          wx.hideLoading();
        }
      },
    });
  });
}

// 处理token失效的函数
function handleTokenExpired(message) {
  // 如果正在处理token失效，避免重复执行
  if (isHandlingTokenExpired) {
    return;
  }

  isHandlingTokenExpired = true;

  // 清除所有登录相关的本地存储（保留账号信息）
  auth.clearAuth(true);

  wx.showToast({
    title: message,
    icon: "none",
    duration: 2000,
  });

  setTimeout(() => {
    isHandlingTokenExpired = false;
    wx.reLaunch({
      url: "/pages/login/index",
    });
  }, 2000);
}

// 导出处理token失效的函数，供其他地方使用
export { handleTokenExpired };
