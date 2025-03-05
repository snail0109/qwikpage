import { ComponentType } from '@/types';
import { Row, Col } from 'antd';
import React from 'react';
import { forwardRef, useImperativeHandle, useState } from 'react';

/*泛型只需要定义组件本身用到的属性*/
export interface IConfig {
  text: string;
}
/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MGrid = ({ id, type, config, elements, children }: ComponentType, ref: any) => {
  const [visible, setVisible] = useState(true);

  // 对外暴露方法
  useImperativeHandle(ref, () => {
    return {
      show() {
        setVisible(true);
      },
      hide() {
        setVisible(false);
      },
    };
  });

  return (
    visible && (
      <Row style={config.style} {...config.props} data-id={id} data-type={type}>
        <Col span={24}>
          {children}
        </Col>
      </Row>
    )
  );
};
export default forwardRef(MGrid);
