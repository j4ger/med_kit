export default class Profile {
  public uuid: string;
  public init: boolean;
  public name: string;
  public delete: boolean;

  public constructor(uuid: string, name: string) {
    this.uuid = uuid;
    this.name = name;
    this.init = true;
    this.delete = false;
  }
}

export function getDisplayName(key: string): string {
  const displayNameDictionary: { [key: string]: string } = {
    uuid: "序列号",
    name: "姓名",
    init: "已注册",
    delete: "已删除",
  };
  return displayNameDictionary[key];
}
