/**
 * 通知类型接口定义
 * 定义了通知组件的各种属性和配置选项
 */
export interface NoticeTypes {
    /**
     * 是否显示通知
     */
    isShow: boolean;
    /**
     * 通知类型标识
     */
    type: string;
    /**
     * 通知内容文本
     */
    content: string;
    /**
     * 是否显示按钮
     */
    isButton: boolean;
    /**
     * 按钮显示的内容文本
     */
    buttonContent: string;
    /**
     * 点击按钮或通知跳转的链接地址
     */
    url: string;
    /**
     * 排行版本信息
     */
    rankVersion: string;
    /**
     * 版本号信息
     */
    version: string;
    /**
     * 通知唯一标识ID
     */
    noticeId: string;
}
