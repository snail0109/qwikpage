import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Tooltip, Image, Card } from 'antd';
import { CopyOutlined, DeleteOutlined, EyeOutlined, FileFilled } from '@ant-design/icons';
import PageIcon from '@/components/icons/PageIcon';
import { openUrl } from '@tauri-apps/plugin-opener';
import dayjs from 'dayjs';
import { message, Modal } from '@/utils/AntdGlobal';
import { pageService } from '@/services';
import { IPage } from '@/types';
import styles from '@/styles/page.module.less';
import pageCardStyle from './index.module.less';

// 页面列表项
const PageCard = ({ list, copy, refresh }: { list: IPage[]; copy: (item: IPage) => void; refresh: () => void }) => {
    const [showPreview, setShowPreview] = useState(false);
    const [previewUrl, setPreviewUrl] = useState('');
    const navigate = useNavigate();

    // 页面操作
    const handleAction = async (type: string, params: IPage) => {
        if (type === 'preview') {
            const previewUrl = `${import.meta.env.VITE_PREVIEW_URL}/project/${params.projectId}${params.path}`;
            openUrl(previewUrl)
            return;
        }

        if (type === 'edit') {
            return navigate(`/editor/${params.projectId}/${params?.id}/edit`);
        }
        if (type === 'copy') {
            return copy?.(params);
        }
        if (type === 'delete') {
            Modal.confirm({
                title: '确认',
                content: '删除后，该页面无法恢复，请谨慎操作。',
                okText: '确认',
                okButtonProps: { danger: true },
                cancelText: '取消',
                onOk: async () => {
                    await pageService.delPageData({
                        id: params.id,
                        projectId: params.projectId
                    });
                    message.success('删除成功');
                    refresh();
                },
            });
        }
    };
    return (
        <>
            <div
                style={{
                    display: 'grid',
                    gridTemplateColumns: `repeat(auto-fill, minmax(320px, 1fr))`,
                    gap: 20,
                }}
            >
                {list.map((item: IPage, index: number) => {
                    return (
                        <Card
                            key={item.id + index}
                            className={pageCardStyle.card}
                            actions={[
                                <Tooltip title="预览">
                                    <EyeOutlined style={{ fontSize: 16 }} onClick={() => handleAction('preview', item)} />
                                </Tooltip>,
                                <Tooltip title="复制">
                                    <CopyOutlined style={{ fontSize: 16 }} onClick={() => handleAction('copy', item)} />
                                </Tooltip>,
                                <Tooltip title="删除">
                                    <DeleteOutlined style={{ fontSize: 16 }} onClick={() => handleAction('delete', item)} />
                                </Tooltip>,
                            ]}
                        >
                            <div className={styles.cardBody} onClick={() => handleAction('edit', item)}>
                                <div className={styles.itemTitle}>
                                    <PageIcon className={styles.pageIcon} />
                                    {item.name}
                                </div>
                                <div className={styles.itemRemark} style={{ height: '40px' }}>{item.remark || '暂无描述'}</div>
                                <div className={styles.updateUser}>
                                    <span>更新于 {dayjs(item.updatedAt).fromNow()}</span>
                                </div>
                            </div>
                        </Card>
                    );
                })}
            </div >
            {/* 图片预览 */}
            < Image
                style={{ display: 'none' }
                }
                preview={{
                    visible: showPreview,
                    src: previewUrl,
                    onVisibleChange: (value) => {
                        setShowPreview(value);
                        setPreviewUrl('');
                    },
                }}
            />
        </>
    );
};

export default PageCard;
