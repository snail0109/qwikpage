import React from 'react';
import { Button } from 'antd';
import styles from './index.module.less';

interface BtnTitle {
  label: string;
  value: string;
}

interface RadioButtonGroupProps {
  disabled?: boolean;
  options: BtnTitle[];
  selected: string;
  onChangeTab?: (tab: any) => void;
}

const RadioButtonGroup: React.FC<RadioButtonGroupProps> = ({ disabled = false, options, selected, onChangeTab }) => {
  return (
    <div className={styles.btnWrap}>
      {options.map((item) => (
        <Button
          disabled={disabled}
          key={item.value}
          autoInsertSpace={false}
          className={`${selected === item.value ? styles.active : ''} ${disabled ? styles.disabled : ''}`}
          onClick={() => onChangeTab && onChangeTab(item)}
        >
          {item.label}
          <div className={styles.checkContainer}></div>
        </Button>
      ))}
    </div>
  );
};

export default RadioButtonGroup;
