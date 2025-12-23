
let env = "develop"

// 防止我们在上传代码的时候，没有将env改成production
const envVersion = wx.getAccountInfoSync().miniProgram.envVersion
if (envVersion === "release" && env !== "production") {
  env = "production"
}

export default {
  env,
  baseUrl: {
    develop: 'http://192.168.0.233:3000/api', 
    production: 'http://api.xxx.com',
  }
}