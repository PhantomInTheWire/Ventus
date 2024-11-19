//
//  ProgressCircleView.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

struct ProgressCircleView: View {
    @ObservedObject var viewModel: SyncViewModel
    let size: CGFloat
    
    var body: some View {
        ZStack {
            // Background effects
            morphingBackground
            glassEffect
            pulseRings
            
            // Progress indicators
            progressTrack
            progressIndicator
            spinningIndicators
            
            // Percentage display
            percentageDisplay
        }
    }
    
    private var morphingBackground: some View {
        Circle()
            .fill(
                RadialGradient(
                    colors: [.blue.opacity(0.15), .clear],
                    center: .center,
                    startRadius: 80,
                    endRadius: 150
                )
            )
            .frame(width: size * 0.7)
            .blur(radius: 20)
            .modifier(MorphingAnimationModifier(phase: viewModel.morphingPhase))
    }
    
    private var glassEffect: some View {
        Circle()
            .fill(.ultraThinMaterial)
            .frame(width: size * 0.55)
            .blur(radius: 5)
    }
    
    private var pulseRings: some View {
        ForEach(0..<3) { index in
            Circle()
                .stroke(
                    LinearGradient(
                        colors: [.blue.opacity(0.2), .purple.opacity(0.1)],
                        startPoint: .top,
                        endPoint: .bottom
                    ),
                    lineWidth: 1
                )
                .frame(width: size * 0.5)
                .scaleEffect(1 + CGFloat(index) * 0.2)
                .opacity(1 - Double(viewModel.progress))
                .blur(radius: CGFloat(index))
                .animation(
                    .easeInOut(duration: 1.5)
                    .repeatForever(autoreverses: false)
                    .delay(Double(index) * 0.5),
                    value: viewModel.progress
                )
        }
    }
    
    private var progressTrack: some View {
        Circle()
            .stroke(.white.opacity(0.1), lineWidth: SyncViewStyle.Circle.progressLineWidth)
            .frame(width: size * 0.5)
            .blur(radius: 0.5)
    }
    
    private var progressIndicator: some View {
        Circle()
            .trim(from: 0, to: CGFloat(viewModel.progress))
            .stroke(
                AngularGradient(
                    colors: [.blue, .purple, .blue],
                    center: .center,
                    startAngle: .degrees(0),
                    endAngle: .degrees(360)
                ),
                style: StrokeStyle(
                    lineWidth: SyncViewStyle.Circle.progressLineWidth,
                    lineCap: .round
                )
            )
            .frame(width: size * 0.5)
            .rotationEffect(.degrees(-90))
            .shadow(color: .blue.opacity(0.3), radius: 5)
    }
    
    private var spinningIndicators: some View {
        ForEach(0..<2) { index in
            Circle()
                .trim(from: 0.4, to: 0.6)
                .stroke(
                    LinearGradient(
                        colors: [.blue.opacity(0.5), .purple.opacity(0.5)],
                        startPoint: .leading,
                        endPoint: .trailing
                    ),
                    style: StrokeStyle(
                        lineWidth: SyncViewStyle.Circle.spinnerLineWidth,
                        lineCap: .round
                    )
                )
                .frame(width: size * (0.6 - Double(index) * 0.1))
                .rotationEffect(.degrees(viewModel.rotation + Double(index) * 180))
        }
    }
    
    private var percentageDisplay: some View {
        AnimatedNumberText(
            number: Int(viewModel.progress * 100),
            font: .system(
                size: SyncViewStyle.Text.percentageSize,
                weight: .bold
            ),
            textColor: .white
        )
        .overlay(
            Text("%")
                .font(.system(
                    size: SyncViewStyle.Text.percentageSymbolSize,
                    weight: .bold
                ))
                .foregroundColor(.white.opacity(0.8))
                .offset(x: 45, y: 1)
        )
        .shadow(color: .blue.opacity(0.3), radius: 5)
    }
}
