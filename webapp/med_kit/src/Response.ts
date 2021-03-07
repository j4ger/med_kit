import Profile from "./Profile";

export interface STDJSONResponse<T> {
  success: boolean;
  data: T;
  errmsg: string;
}

export interface CreateData {
  uuid: string;
}

export interface GetData {
  profile: Profile;
}

export interface VerifyData {
  username: string;
}

export interface LoginData {
  JWT: boolean;
}

export interface RegisterData {
  username: string;
}

export interface QueryData {
  exist: boolean;
  init: boolean;
}

export interface UpdateData {
  exist: boolean;
  init: boolean;
  updated_count: number;
}

export interface InitData {
  exist: boolean;
}
