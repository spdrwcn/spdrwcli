#include <ArduinoJson.h>

void setup() {
  Serial.setRxBufferSize(2048);
  Serial.setTxBufferSize(2048);
  Serial.begin(115200);
  Serial.println("等待指令...");
}

void loop() {
  if (Serial.available()) {
    StaticJsonDocument<2048> doc;
    DeserializationError error = deserializeJson(doc, Serial);

    while (Serial.available()) Serial.read();

    if (error) {
      Serial.print("JSON 解析失败: ");
      Serial.println(error.c_str());
      return;
    }
    const char* type = doc["type"];
    const char* cmd = doc["cmd"];
    const char* addr = doc["addr"];
    const char* value = doc["value"];
    const char* number = doc["number"];
    
    Serial.println(type);
    Serial.println(cmd);
    Serial.println(addr);
    Serial.println(value);
    Serial.println(number);
  }
}
