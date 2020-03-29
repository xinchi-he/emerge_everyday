package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
	"sort"
	"strconv"
	"strings"
	"time"
	"unicode"
)

var start int
var duration int
var emerges map[int]int
var packages map[string]int
var monthlyEmerges map[string]int
var verbose bool

func main() {
	verbosePtr := flag.Bool("v", false, "verbose")
	flag.Parse()

	verbose = *verbosePtr

	copyLog()
	parseLog("/tmp/emerge.log")
	printDictionary(emerges, "America/Chicago") // <- your timezone
}

func copyLog() bool {
	cmd := exec.Command("sudo", "./utils/copy_log.sh")

	_, err := cmd.Output()
	if err != nil {
		log.Fatal(err)
		return false
	}
	return true
}

func parseLog(filename string) {
	emerges = make(map[int]int)
	packages = make(map[string]int)

	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		splits := strings.Split(scanner.Text(), " ")

		if strings.Contains(scanner.Text(), "Started") {
			start, err = strconv.Atoi(strings.Split(splits[0], ":")[0])
			if err != nil {
				log.Fatal(err)
			}
		} else if strings.Contains(scanner.Text(), "terminating") {
			end, err := strconv.Atoi(strings.Split(splits[0], ":")[0])
			if err != nil {
				log.Fatal(err)
			}
			duration = end - start
			emerges[start] = duration
		} else if strings.Contains(scanner.Text(), ">>>") && strings.Contains(scanner.Text(), "emerge") {
			packageName := splits[7]
			pName := ""
			for i, c := range packageName {
				next := []rune(packageName)[i+1]
				if c == '-' && unicode.IsDigit(next) {
					break
				}

				pName += string(c)
			}

			if _, ok := packages[pName]; ok {
				packages[pName]++
			} else {
				packages[pName] = 1
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func printDictionary(dictionary map[int]int, timezone string) {
	monthlyEmerges = make(map[string]int)

	loc, _ := time.LoadLocation(timezone)

	var keys []int
	for k := range dictionary {
		keys = append(keys, k)
	}

	sort.Ints(keys)

	for _, k := range keys {
		tm := time.Unix(int64(k), 0).In(loc)
		year, month, _ := tm.Date()
		var dateIndex string

		if int(month) < 10 {
			dateIndex = "0" + strconv.Itoa(int(month)) + "/" + strconv.Itoa(year)
		} else {
			dateIndex = strconv.Itoa(int(month)) + "/" + strconv.Itoa(year)
		}

		if _, ok := monthlyEmerges[dateIndex]; ok {
			monthlyEmerges[dateIndex]++
		} else {
			monthlyEmerges[dateIndex] = 1
		}

		if verbose {
			if dictionary[k] < 60 {
				fmt.Println(fmt.Sprintf("Started at: %s, duration: %d sec(s)", tm, dictionary[k]))
			} else if dictionary[k] > 60 && dictionary[k] < 3600 {
				fmt.Println(fmt.Sprintf("Started at: %s, duration: %d min(s) %d sec(s)", tm, dictionary[k]/60, dictionary[k]%60))
			} else if dictionary[k] > 3600 {
				fmt.Println(fmt.Sprintf("Started at: %s, duration: %d hr(s) %d min(s) %d sec(s)", tm, dictionary[k]/3600, dictionary[k]%3600/60, dictionary[k]%3600%60))
			}
		}

	}

	fmt.Println("********* Monthly Emerges **********")
	for _, v := range rankByWordCount(monthlyEmerges) {
		fmt.Println(fmt.Sprintf("%s,  emerges: %d", v.Key, v.Value))
	}

	fmt.Println()

	fmt.Println("********** Top Emerged Packages (more than 10 times) **********")
	for _, v := range rankByWordCount(packages) {
		if v.Value >= 10 {
			fmt.Println(v.Key, v.Value)
		}
	}

}

/*
	helper functions
*/
func rankByWordCount(wordFrequencies map[string]int) PairList {
	pl := make(PairList, len(wordFrequencies))
	i := 0
	for k, v := range wordFrequencies {
		pl[i] = Pair{k, v}
		i++
	}
	sort.Sort(sort.Reverse(pl))
	return pl
}

type Pair struct {
	Key   string
	Value int
}

type PairList []Pair

func (p PairList) Len() int           { return len(p) }
func (p PairList) Less(i, j int) bool { return p[i].Value < p[j].Value }
func (p PairList) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }
