package main

import "fmt"

func main() {
	var log string = "１５；００　レクリエーション開始"
	fmt.Println("現在の日誌：", log)

	updateLog(&log)

	fmt.Println("変更後の日誌：", log)
}

func updateLog(logPtr *string) {
	*logPtr = *logPtr + "（全員参加で盛り上がりました）"
}
