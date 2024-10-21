export interface IUser {
    uid: number,
    login: string,
    name?: string,
    display_name?: string,
    full_name?: string,
    sex: string,
    verified?: boolean,
    regions: number[],
}