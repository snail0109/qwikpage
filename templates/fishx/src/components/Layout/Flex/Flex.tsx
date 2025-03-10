import { ComponentType, MaterialProp } from '@/types';
import { Flex } from 'antd';
import React from 'react';
import { forwardRef, useImperativeHandle, useState } from 'react';
import Material from '@/components/Material';

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
const MFlex = ({ id, type, config, children }: ComponentType, ref: any) => {
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

  const processedChildren = React.Children.map(children, (child) => {
    if (!React.isValidElement(child)) return child;

    // 获取子组件原始 props
    const childProps = child.props as MaterialProp;

    return (
      <Material
        {...childProps} // 传递原始配置
      >
        {child}
      </Material>
    );
  });

  return (
    visible && (
      <Flex style={config.style} {...config.props} data-id={id} data-type={type}>
        {processedChildren}
      </Flex>
    )
  );
};
export default forwardRef(MFlex);
