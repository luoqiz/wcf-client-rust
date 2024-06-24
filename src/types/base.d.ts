export type BaseResponse<T> = {
    status: number;
    error: string;
    data: T
}