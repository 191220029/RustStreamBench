package main

import (
	"fmt"
	"github.com/trustmaster/goflow"
	"os/exec"
)

type Tricker struct {
	Sig            <-chan string
	Out0            chan<- string
	Out1            chan<- string
	Out2            chan<- string
	Out3            chan<- string
	Out4            chan<- string
	Out5            chan<- string
	Out6            chan<- string
	Out7            chan<- string
	Out8            chan<- string
	Out9            chan<- string
}
// Send Trick signal
func (c *Tricker) Process() {
	// Keep reading incoming packets
	for sig := range c.Sig {
		fmt.Println("Tricker Received sig", sig)
		c.Out0 <- "0"
		c.Out1 <- "1"
		c.Out2 <- "2"
		c.Out3 <- "3"
		c.Out4 <- "4"
		c.Out5 <- "5"
		c.Out6 <- "6"
		c.Out7 <- "7"
		c.Out8 <- "8"
		c.Out9 <- "9"
	}
}

// Trainer sends greetings
type Trainer struct {
	Class           <-chan string // input port
	Res            chan<- string // output port
}

// Process incoming data
func (c *Trainer) Process() {
	// Keep reading incoming packets
	for class := range c.Class {
		fmt.Println("Trainer Class=", class)
		cmd := exec.Command("python", "-B", "/home/xiaolongfu/dagrs-perf/dagrs-NJU-fxl/examples/dagrs-sklearn/examples/minist_i.py", "", class)
		output, _ := cmd.CombinedOutput()
		// Send the greeting to the output port
		
		fmt.Println("Trainer Class", class, "finished")
		c.Res <- string(output[:])
	}
}

// Verifier prints its input on screen
type Verifier struct {
	Line <-chan string // inport
}

// Process prints a line when it gets it
func (c *Verifier) Process() {
	var t = 0
	for line := range c.Line {
		t += 1
		fmt.Println(line)
		if t == 10 {
			fmt.Println("All train finished")
			cmd := exec.Command("python", "-B", "/home/xiaolongfu/dagrs-perf/dagrs-NJU-fxl/examples/dagrs-sklearn/examples/minist_root.py", "")
			output, _ := cmd.CombinedOutput()
			// Send the greeting to the output port
			fmt.Println("Sending output", string(output[:]))
		}
	}
}

// NewGreetingApp defines the app graph
func NewGreetingApp() *goflow.Graph {
	n := goflow.NewGraph()
	// Add processes to the network
	n.Add("Tricker", new(Tricker))
	n.Add("Trainer0", new(Trainer))
	n.Add("Trainer1", new(Trainer))
	n.Add("Trainer2", new(Trainer))
	n.Add("Trainer3", new(Trainer))
	n.Add("Trainer4", new(Trainer))
	n.Add("Trainer5", new(Trainer))
	n.Add("Trainer6", new(Trainer))
	n.Add("Trainer7", new(Trainer))
	n.Add("Trainer8", new(Trainer))
	n.Add("Trainer9", new(Trainer))
	n.Add("Verifier", new(Verifier))
	n.Connect("Tricker", "Out0", "Trainer0", "Class")
	n.Connect("Tricker", "Out1", "Trainer1", "Class")
	n.Connect("Tricker", "Out2", "Trainer2", "Class")
	n.Connect("Tricker", "Out3", "Trainer3", "Class")
	n.Connect("Tricker", "Out4", "Trainer4", "Class")
	n.Connect("Tricker", "Out5", "Trainer5", "Class")
	n.Connect("Tricker", "Out6", "Trainer6", "Class")
	n.Connect("Tricker", "Out7", "Trainer7", "Class")
	n.Connect("Tricker", "Out8", "Trainer8", "Class")
	n.Connect("Tricker", "Out9", "Trainer9", "Class")
	// Connect them with a channel
	n.Connect("Trainer0", "Res", "Verifier", "Line")
	n.Connect("Trainer1", "Res", "Verifier", "Line")
	n.Connect("Trainer2", "Res", "Verifier", "Line")
	n.Connect("Trainer3", "Res", "Verifier", "Line")
	n.Connect("Trainer4", "Res", "Verifier", "Line")
	n.Connect("Trainer5", "Res", "Verifier", "Line")
	n.Connect("Trainer6", "Res", "Verifier", "Line")
	n.Connect("Trainer7", "Res", "Verifier", "Line")
	n.Connect("Trainer8", "Res", "Verifier", "Line")
	n.Connect("Trainer9", "Res", "Verifier", "Line")
	// Our net has 1 inport mapped to Trainer.Name
	n.MapInPort("In", "Tricker", "Sig")
	return n
}

func main() {
	// Create the network
	net := NewGreetingApp()
	// We need a channel to talk to it
	in := make(chan string)
	net.SetInPort("In", in)
	// Run the net
	wait := goflow.Run(net)
	// Now we can send some names and see what happens
	in <- "John"
	// Send end of input
	close(in)
	// Wait until the net has completed its job
	<-wait
}