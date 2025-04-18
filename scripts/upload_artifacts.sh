#!/bin/bash
set -e

# 检查必要的环境变量
if [ -z "$ALIYUN_USERNAME" ] || [ -z "$ALIYUN_PASSWORD" ] || [ -z "$ALIYUN_BASE_URL" ] || [ -z "$APP_VERSION" ] || [ -z "$ARTIFACT_PATHS" ] || [ -z "$JSON_PATH" ]; then
  echo "Error: Required environment variables are not set"
  echo "ALIYUN_USERNAME: ${ALIYUN_USERNAME:-(not set)}"
  echo "ALIYUN_PASSWORD: ${ALIYUN_PASSWORD:+set (value hidden)}"
  echo "ALIYUN_BASE_URL: ${ALIYUN_BASE_URL:-(not set)}"
  echo "APP_VERSION: ${APP_VERSION:-(not set)}"
  echo "ARTIFACT_PATHS: ${ARTIFACT_PATHS:-(not set)}"
  echo "JSON_PATH: ${JSON_PATH:-(not set)}"
  exit 1
fi

# 将 JSON 字符串转换为数组
IFS=' ' read -ra ARTIFACT_PATHS_ARRAY <<< "$ARTIFACT_PATHS"
# ARTIFACT_PATHS_ARRAY=($(echo "$ARTIFACT_PATHS" | jq -r '.[]'))

# 从文件路径中提取app_name
extract_app_name() {
  local filepath=$1
  local filename=$(basename "$filepath")

  # 根据文件名判断平台和架构
  if [[ "$filename" == *"_x64-setup.exe" ]]; then
    echo "QwikPage-windows-x64.exe"
  else
    # 使用原始文件名
    echo "$filename"
  fi
}

# 上传函数
upload_file() {
  local file=$1
  local app_name=$(extract_app_name "$file")

  # 构建上传路径
  UPLOAD_URL="${ALIYUN_BASE_URL}?version=${APP_VERSION}&fileName=${app_name}"
  
  echo "Uploading to: $UPLOAD_URL"
  
  # 使用 curl 发送 POST 请求
  RESPONSE=$(curl -s -w "%{http_code}" \
    -u "${ALIYUN_USERNAME}:${ALIYUN_PASSWORD}" \
    -XPOST "${UPLOAD_URL}" \
    -F "file=@${file}")
  
  HTTP_STATUS=${RESPONSE: -3}
  RESPONSE_BODY=${RESPONSE:0:${#RESPONSE}-3}
  
  echo "HTTP Status for $app_name: $HTTP_STATUS"
  echo "Response for $app_name: $RESPONSE_BODY"
  
  if [[ ! "$HTTP_STATUS" =~ ^2[0-9][0-9]$ ]]; then
    echo "Upload failed for $app_name with status $HTTP_STATUS"
    return 1
  fi
  
  echo "Upload successful for $app_name!"
  return 0
}

# 删除函数
delete_file() {
  local file=$1
  local app_name=$(extract_app_name "$file")

  echo "Deleting existing $app_name from server"
  DELETE_URL="${ALIYUN_BASE_URL}/$app_name?version=$APP_VERSION"

  # 使用curl发送 DELETE 请求
  DELETE_RESPONSE=$(curl -s -w "%{http_code}" \
    -u "${ALIYUN_USERNAME}:${ALIYUN_PASSWORD}" \
    -X DELETE "${DELETE_URL}")

  DELETE_HTTP_STATUS=${DELETE_RESPONSE: -3}
  DELETE_RESPONSE_BODY=${DELETE_RESPONSE:0:${#DELETE_RESPONSE}-3}

  echo "Delete HTTP Status for $app_name: $DELETE_HTTP_STATUS"
  echo "Delete Response for $app_name: $DELETE_RESPONSE_BODY"
}

# 删除json
delete_json() {
  DELETE_URL="${ALIYUN_BASE_URL}/latest.json?version=0"

  # 使用curl发送 DELETE 请求
  DELETE_RESPONSE=$(curl -s -w "%{http_code}" \
    -u "${ALIYUN_USERNAME}:${ALIYUN_PASSWORD}" \
    -X DELETE "${DELETE_URL}")

  DELETE_HTTP_STATUS=${DELETE_RESPONSE: -3}
  DELETE_RESPONSE_BODY=${DELETE_RESPONSE:0:${#DELETE_RESPONSE}-3}

  echo "Delete HTTP Status for $app_name: $DELETE_HTTP_STATUS"
  echo "Delete Response for $app_name: $DELETE_RESPONSE_BODY"
}

# 上传json
upload_json() {
  local json_content=$(cat "$JSON_PATH")
  local base_url_origin="http://47.122.78.114:23432/update/Qwikpage/bin/${APP_VERSION}"
  
  # 修改 JSON 文件内容
  local modified_json=$(echo "$json_content" | jq \
    --arg version "$APP_VERSION" \
    --arg base_url "$base_url_origin" \
    '.version = $version | 
     .platforms["windows-x86_64"].url = ($base_url + "/QwikPage_x64-setup.exe") |
     .platforms["darwin-x86_64"].url = ($base_url + "/QwikPage_x64.app.tar.gz") |
     .platforms["darwin-aarch64"].url = ($base_url + "/QwikPage_aarch64.app.tar.gz")')

  # 将修改后的内容写入临时文件
  echo "$modified_json" > "${JSON_PATH}.tmp"

  # 构建上传路径
  UPLOAD_URL="${ALIYUN_BASE_URL}?version=0&fileName=latest.json"

  # 上传修改后的文件
  RESPONSE=$(curl -s -w "%{http_code}" \
    -u "${ALIYUN_USERNAME}:${ALIYUN_PASSWORD}" \
    -XPOST "${UPLOAD_URL}" \
    -F "file=@${JSON_PATH}.tmp")
  
  HTTP_STATUS=${RESPONSE: -3}
  RESPONSE_BODY=${RESPONSE:0:${#RESPONSE}-3}
  
  echo "HTTP Status for latest.json: $HTTP_STATUS"
  echo "Response for latest.json: $RESPONSE_BODY"

  # 清理临时文件
  rm "${JSON_PATH}.tmp"

  if [[ ! "$HTTP_STATUS" =~ ^2[0-9][0-9]$ ]]; then
    echo "Upload failed for latest.json with status $HTTP_STATUS"
    return 1
  fi
  
  echo "Upload successful for latest.json!"
  return 0
}

echo "Starting artifact upload process..."

# 异步删除所有文件
echo "Starting asynchronous deletion..."
for path in "${ARTIFACT_PATHS_ARRAY[@]}"; do
  echo "Processing delete $path"
  delete_file "$path" &
done

# 等待所有删除操作完成
wait
echo "All deletion operations completed"

# 添加延迟
echo "Waiting for 5 seconds after delete operations..."
# sleep 5

# 异步上传所有二进制文件
echo "Starting asynchronous uploads..."
pids=()
for path in "${ARTIFACT_PATHS_ARRAY[@]}"; do
  echo "Processing upload $path"
  upload_file "$path" &
  pids+=($!)
done

# 等待所有上传操作完成并检查结果
failed=0
for pid in "${pids[@]}"; do
  if ! wait $pid; then
    failed=$((failed+1))
    echo "Upload failed for PID: $pid"
  fi
done

if [ $failed -gt 0 ]; then
  echo "Error: $failed uploads failed"
  exit 1
fi

echo "All artifacts have been processed successfully!"

# 处理 JSON 文件
echo "Processing JSON file..."
delete_json
echo "Waiting for 2 seconds after JSON deletion..."
sleep 2
upload_json || exit 1

echo "All artifacts have been processed successfully!"
