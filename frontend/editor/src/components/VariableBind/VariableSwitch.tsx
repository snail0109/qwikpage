import React, { useState, useMemo } from "react";
import { Form, Switch } from "antd";
import VariableBind from "./VariableBind";
import FunctionIcon from "@/assets/icons/FunctionIcon.svg?react";
import styles from "./variable.module.less";

interface VariableSwitchProps {
  label?: string;
  name?: (string | number)[];
  initialValue?: boolean | { type: 'static' | 'variable'; value: any };
}

const VariableSwitch: React.FC<VariableSwitchProps> = ({ label, name, initialValue = false }) => {
  const [isEditing, setIsEditing] = useState(false);
  // const [tempValue, setTempValue] = useState(""); // 临时存储VariableBind的值
  // const form = Form.useFormInstance();
  // const fieldName = name[0];

  const handleToggleEdit = () => {
    setIsEditing(true);
  };

  const handleEndEdit = () => {
    setIsEditing(false);
  };

  return (
    <Form.Item
      className={styles.variableSwitch}
      name={name}
      initialValue={initialValue}
      colon={false}
      layout={isEditing ? "vertical" : "horizontal"}
      labelAlign="left"
      label={
        <div style={{ display: "flex", justifyContent: "space-between", width: "100%", alignItems: "center" }}>
          <span>{label}</span>
          {isEditing && (
            <FunctionIcon
              onClick={handleEndEdit}
              style={{
                color: "#ffffff",
                backgroundColor: "#216EF7",
                border: "unset",
                width: 15,
                height: 15,
              }}
              className={styles.variableIcon}
            />
          )}
        </div>
      }
    >
      {!isEditing ? (
        <div style={{ display: "flex", alignItems: "center" }}>
          <Form.Item name={name} noStyle valuePropName="checked">
            <Switch size="small" />
          </Form.Item>
          <FunctionIcon
            onClick={handleToggleEdit}
            style={{ color: "#999999", marginLeft: 8, width: 15, height: 15 }}
            className={styles.variableIcon}
          />
        </div>
      ) : (
        <Form.Item name={name} noStyle>
          <VariableBind />
        </Form.Item>
      )}
    </Form.Item>
  );
};

export default VariableSwitch;