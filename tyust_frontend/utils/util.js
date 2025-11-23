function getNowWeek(startDate, totalWeek) {
  const nowDate = new Date().getTime()
  startDate = new Date(startDate)
  const time = nowDate - startDate
  let nowWeek = Math.ceil(time / 1000 / 60 / 60 / 24 / 7)
  if (nowWeek <= 0) {
    nowWeek = 1
  }
  if (nowWeek > totalWeek) {
    nowWeek = totalWeek
  }
  return nowWeek
}

module.exports = {
  getNowWeek
}