import React from 'react';
import { Radio } from 'antd';
import styles from "./index.module.less";

interface ColorOption {
  label: string;
  value: string;
}

interface ColorRadioGroupProps {
  options?: ColorOption[]; // 使 options 可选
  disabled?: boolean;
  selectedValue: string;
  onChange: (value: string) => void;
}

const defaultColorOptions: ColorOption[] = [
  { label: 'blue', value: 'blue' },
  { label: 'purple', value: 'purple' },
  { label: 'red', value: 'red' },
  { label: 'green', value: 'green' },
];

const ColorRadioGroup: React.FC<ColorRadioGroupProps> = ({
  options = defaultColorOptions,
  disabled = false,
  selectedValue,
  onChange,
}) => {
  return (
    <Radio.Group
      optionType='button'
      value={selectedValue}
      className={styles.themeColor}
      onChange={(e) => onChange(e.target.value)}
      disabled={disabled}
    >
      {options.map((option) => (
        <Radio key={option.value} value={option.value}>
          <div
            style={{
              backgroundColor: option.value,
              padding: '15px',
              borderWidth: '2px',
              borderStyle: 'solid',
              borderColor: selectedValue === option.value ? '#D3E4FF' : option.value,
              borderRadius: '4px',
            }}
          />
        </Radio>
      ))}
    </Radio.Group>
  );
};

export default ColorRadioGroup;
