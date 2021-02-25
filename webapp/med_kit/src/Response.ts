export interface STDJSONResponse<T> {
  success: boolean;
  data: T;
}

export interface CreateData {
  uuid: string;
}

export interface GetData<T> {
  exist: boolean;
  init: boolean;
  profile: T;
}

export interface UpdateData {
  exist: boolean;
  init: boolean;
  updated_count: number;
}

export interface InitData {
  exist: boolean;
}
