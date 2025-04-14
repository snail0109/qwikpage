import React, { useState, useMemo } from 'react';
import { Segmented, Select } from 'antd';
import * as icons from '@qwikpage/icons';
import { styled } from 'styled-components';
import QIcon from './QIcon';

const BoxWrapper = styled.div`
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  height: 300px;
  overflow: auto;
`;

const IconItem = styled.div`
  width: 50px;
  height: 50px;
  padding: 10px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
`;

/**
 * 菜单中，自定义图标选择列表
 */
export default function QIconList({ value, onChange }: any) {
  const [open, setOpen] = useState(false);
  const [type, setType] = useState('线框风格');

  const allList = useMemo(() => {
    // 获取所有的qwikpage图标，动态渲染到下拉框中
    const iconsList: { [key: string]: any } = icons;

    return Object.keys(iconsList)
      .filter((item) => !['default', 'renderIconDefinitionToSVGElement'].includes(item))
      .map((key) => ({
        value: key,
        label: <QIcon name={key} style={{ fontSize: '24px' }} />
      }));
  }, []);

  // 过滤出不同风格的图标
  const outlined = useMemo(() => allList.filter((item) => item.value.endsWith('Outlined')), [allList]);
  const filled = useMemo(() => allList.filter((item) => item.value.endsWith('Filled')), [allList]);
  const twoTone = useMemo(() => allList.filter((item) => item.value.endsWith('TwoTone')), [allList]);
  const color = useMemo(() => allList.filter((item) => item.value.endsWith('Color')), [allList]);

  const options = useMemo(() => {
    switch (type) {
      case '线框风格':
        return outlined;
      case '实底风格':
        return filled;
      case '双色风格':
        return twoTone;
      default:
        return color;
    }
  }, [type, outlined, filled, twoTone, color]);

  return (
    <Select
      placeholder="请选择图标"
      // allowClear
      value={value ? {
        value: value,
        label: <QIcon name={value} style={{ fontSize: "24px" }} />
      } : undefined} 
      open={open}
      options={options}
      onDropdownVisibleChange={(visible) => setOpen(visible)}
      // onClear={() => onChange('')}
      dropdownRender={() => (
        <div>
          <Segmented
            options={['线框风格', '实底风格', '双色风格', '彩色']}
            block
            value={type}
            onChange={setType}
          />
          <BoxWrapper>
            {options.map((item) => {
              return (
                <IconItem
                  key={item.value}
                  onClick={() => {
                    onChange(item.value);
                    setOpen(false);
                  }}
                >
                  {item.label}
                </IconItem>
              );
            })}
          </BoxWrapper>
        </div>
      )}
    />
  );
}