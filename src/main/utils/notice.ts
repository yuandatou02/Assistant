import type {NoticeTypes} from "@/main/types/notice";
import {h} from "vue";
import {useDialog} from "naive-ui";
import type {DialogApiInjection} from "naive-ui/es/dialog/src/DialogProvider";
import {requestFetch} from "@/main/utils/request.ts";

declare const __APP_VERSION__: string;

// 通知类
export class Notice {
    public url = "https://frank-notice-1302853015.cos.ap-chongqing.myqcloud.com/frankRust.json";
    public dialog = useDialog();
    public notice: null | NoticeTypes = null;

    public async init(): Promise<boolean> {
        const timestamp = new Date().getTime();
        const res = await requestFetch<NoticeTypes>(this.url + `?date=${timestamp}`, "GET");
        if (res === null) {
            return false;
        }
        this.notice = res as NoticeTypes;
        localStorage.setItem("rankVersion", this.notice.rankVersion);
        if (!this.notice.isShow) {
            // this.showUpdate(this.notice.version)
            return false;
        } else if (localStorage.getItem("oldNoticeId") === this.notice.noticeId) {
            return true;
        } else {
            localStorage.setItem("noticeId", this.notice.noticeId);
            this.showDialog(false);
            return true;
        }
    }

    /**
     * 显示通知对话框
     * @param isVersion - 可选参数，是否显示版本更新对话框，默认为false
     * @returns void
     */
    public showDialog(isVersion?: boolean) {
        const notice = this.notice as NoticeTypes;
        const versionInfo = `当前版本${__APP_VERSION__}，最新版本[ ${this.notice?.version} ↑]  请立即更新，获取最佳体验！`;

        // 构建对话框内容的虚拟节点
        const contentVNode = () => {
            const content = isVersion ? versionInfo + notice.content : notice.content;
            const textList = content.split("/n");
            return textList.map((text: string) => {
                return h("p", [text]);
            });
        };
        this.dialog[notice.type as keyof DialogApiInjection]({
            title: isVersion ? "版本更新" : "新的通知",
            content: contentVNode,
            showIcon: true,
            maskClosable: true,
            closable: false,
            autoFocus: false,
            style: "margin:8px;max-width:334px",
            positiveText: isVersion ? "点击下载" : notice.buttonContent,
            negativeText: isVersion ? "推荐更新" : "不再提醒",
            onPositiveClick: () => {
                open(notice.url);
            },
            onNegativeClick: () => {
                localStorage.setItem("oldNoticeId", notice.noticeId);
            }
        });
    }
}