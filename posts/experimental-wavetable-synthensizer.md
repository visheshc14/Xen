---
title: Project - Experimental Wavetable Synthensizer
tags: [webassembly, rust, DSP, audioengineering, web-audio, wavetable-synthesizer]
date: 2025-10-09
blurb: "Aeon Keys is a Full-Stack Audio Synthesis Project that I Designed and Implemented to Explore the Intersection of High-Performance Systems Programming and Creative Audio Technology. Inspired by Industry-Standard Soft-Synths like Xfer Serum, Aeon Keys Reimagines a Wavetable Synthesizer as a Browser-Native Application using Rust, WebAssembly and the Web Audio API."
---

#  Aeon Keys - Serum-Like Synthesizer 

A browser-based **Software Synthesizer** Inspired by Xfer Serum, Built using **Rust (Compiled to WebAssembly)** and the **Web Audio API**.  
It Provides a **Modern Synth UI** with **Oscillators, Filters, Envelopes, LFOs, Effects, Modulation Matrix, Wavetable Editor, Presets, MIDI Input, and Spectrum Analyzers** — All Running **Directly in Your Browser**.

I Designed and Implemented the Core Synthesis Engine in Rust, Compiled to WebAssembly for Ultra-Low Latency Performance in the Browser. My Contribution Spanned the Entire Stack  from DSP Algorithm Design (Oscillators, Filters, ADSR, LFOs, Modulation Matrix) to the Frontend UI Integration using HTML, TailwindCSS, and Vanilla JS.

I Built the Real-Time Audio Pipeline, Created a Custom Wavetable Editor, Integrated MIDI Input, and Added Audio Effects (Reverb, Delay, EQ). I Also Handled Cross-Language Data Exchange Between Rust and JavaScript, Optimized Rendering with Typed Arrays, and Implemented a Spectrum Analyzer and Recording System.

Beyond Coding, I Designed the UI/UX, Focusing on a Responsive, Serum-Like Interface that Feels Like a Real Software Synthesizer Interactive Knobs, Visual Feedback, and Modulation Routing That All Sync in Real Time.

---

[Aeon-Keys - Source Code](https://github.com/visheshc14/Aeon-Keys)

---
<img width="637" height="1006" alt="Screenshot 2025-10-09 at 2 56 29 PM" src="https://github.com/user-attachments/assets/2c267efa-891a-4479-a0eb-4b334a9c8e39" />

---

##  Features

###  Oscillators
- Dual oscillator setup (OSC A / OSC B)
- Waveforms: **Sine, Saw, Square, Triangle, Noise, Wavetable**
- Per-oscillator **detune** and **gain**
- Wavetable loading with custom editor

###  Wavetable Editor
- **Freehand drawing** mode  
- **Additive synthesis** mode (harmonic sliders)  
- Normalize, Clear, Save & Load slots  
- Real-time **FFT preview**  

###  Filters & Envelopes
- **Filter types:** (Low-pass, HP, BP, etc. – extensible)  
- **Cutoff & Resonance knobs**  
- **ADSR Envelopes** (Attack, Decay, Sustain, Release)  
- Envelope modulation to filter cutoff  

###  LFOs
- Waveforms: Sine, Saw, Square, Triangle, Random  
- Adjustable **rate & depth**  
- Retrigger toggle  
- Routable via modulation matrix  

###  Modulation Matrix
- Route **LFOs / Envelopes** to filter cutoff  
- Extensible for more destinations  

###  Effects
- **Delay**: time, feedback, wet/dry  
- **Convolution Reverb**: IR-based space simulation  
- **Master Gain**  

###  MIDI Integration
- Full **WebMIDI support** (Chrome/Edge, Safari experimental)  
- Play with your **MIDI keyboard** in real-time  

###  Visualizers
- **Spectrum Analyzer** (real-time FFT)  
- **Wavetable waveform + frequency preview**  

###  Presets
- Save/load presets with **LocalStorage**  
- Quick recall of custom patches  
---

##  Getting Started

### 1. Build Rust → WASM
```bash
# Install wasm-pack if missing
cargo install wasm-pack

# Build the Rust synth backend
wasm-pack build --target web --out-dir ./static/pkg
```

### 2. Run a Local Dev Server 
```bash
# Using Python
cd static
python3 -m http.server 8000

# OR using Node
npx serve static
```
