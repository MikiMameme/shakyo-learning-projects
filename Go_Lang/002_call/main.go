package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup

	staffs := []string{"佐藤", "鈴木", "高橋"}

	fmt.Println("ナースコールが鳴りました")

	for _, name := range staffs {
		wg.Add(1)
		go func(n string) {
			defer wg.Done()
			handleCall(n)
		}(name)
	}
	wg.Wait()
	fmt.Println("対応が完了しました")
}
func handleCall(name string) {
	fmt.Printf("%sさんが部屋へ向かっています...\n", name)
	time.Sleep(2 * time.Second)
	fmt.Printf("%sさんの対応が終わりました\n", name)
}
