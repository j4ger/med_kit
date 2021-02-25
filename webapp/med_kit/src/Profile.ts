export default class Profile {
  public uuid: string;
  public init: boolean;
  public name: string;
  public delete: boolean;
  public gender: string;
  public time: string;
  public phone: string;
  public address: string;
  public age: number;
  public email: string;
  public hostpital: string;

  public constructor(
    uuid: string,
    name: string,
    gender: string,
    time: string,
    phone: string,
    address: string,
    age: number,
    email: string,
    hostpital: string
  ) {
    this.uuid = uuid;
    this.name = name;
    this.init = true;
    this.delete = false;
    this.gender = gender;
    this.time = time;
    this.phone = phone;
    this.address = address;
    this.age = age;
    this.email = email;
    this.hostpital = hostpital;
  }
}

export function getDisplayName(key: string): string {
  const displayNameDictionary: { [key: string]: string } = {
    uuid: "序列号",
    name: "姓名",
    init: "已注册",
    delete: "已删除",
    gender: "性别",
    time: "取样时间",
    phone: "电话号码",
    address: "地址",
    age: "年龄",
    email: "电子邮件地址",
    hostpital: "送检医院",
  };
  return displayNameDictionary[key];
}
