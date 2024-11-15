//
//  SyncView.swift
//  Ventus-iOS
//
//  Created by Karan Haresh Lokchandani on 15/11/24.
//

import SwiftUI


struct SyncView: View {
    @State private var progress: Float = 0.0
    @State private var rotation: Double = 0.0
    @State private var showingDoneView = false

    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)),
        Color(#colorLiteral(red: 0.1, green: 0.1, blue: 0.3, alpha: 1))
    ]

    var body: some View {
        ZStack {
            AnimatedBackgroundView(gradientColors: gradientColors)
            RadialGradient(
                gradient: Gradient(colors: gradientColors),
                center: .center,
                startRadius: 100,
                endRadius: 400
            )
            .ignoresSafeArea()
            .overlay(
                GeometryReader { geometry in
                    ForEach(0..<20) { _ in
                        Circle()
                            .fill(Color.white.opacity(0.1))
                            .frame(width: CGFloat.random(in: 50...150))
                            .position(
                                x: CGFloat.random(in: 0...geometry.size.width),
                                y: CGFloat.random(in: 0...geometry.size.height)
                            )
                            .blur(radius: 20)
                    }
                }
            )

            VStack(spacing: 40) {
                Text("Syncing Files...")
                    .font(.system(size: 28, weight: .bold))
                    .foregroundStyle(
                        LinearGradient(
                            gradient: Gradient(colors: [Color.blue, Color.purple, Color.teal]),
                            startPoint: .leading,
                            endPoint: .trailing
                        )
                    )

                // Circular Progress Indicator
                ZStack {
                    // Outer glow
                    Circle()
                        .stroke(Color.white.opacity(0.1), lineWidth: 20)
                        .frame(width: 200, height: 200)
                        .blur(radius: 20)

                    // Background track
                    Circle()
                        .stroke(Color.white.opacity(0.1), lineWidth: 15)
                        .frame(width: 200, height: 200)

                    // Progress track
                    Circle()
                        .trim(from: 0, to: CGFloat(progress))
                        .stroke(
                            LinearGradient(
                                gradient: Gradient(colors: [.blue, .purple, .teal]),
                                startPoint: .leading,
                                endPoint: .trailing
                            ),
                            style: StrokeStyle(lineWidth: 15, lineCap: .round)
                        )
                        .frame(width: 200, height: 200)
                        .rotationEffect(.degrees(-90))

                    // Spinning outer ring with gradient
                    Circle()
                        .trim(from: 0.4, to: 0.6)
                        .stroke(
                            LinearGradient(
                                gradient: Gradient(colors: [.blue.opacity(0.5), .purple.opacity(0.5)]),
                                startPoint: .leading,
                                endPoint: .trailing
                            ),
                            style: StrokeStyle(lineWidth: 2, lineCap: .round)
                        )
                        .frame(width: 230, height: 230)
                        .rotationEffect(.degrees(rotation))
                        .onAppear {
                            withAnimation(.linear(duration: 2).repeatForever(autoreverses: false)) {
                                rotation = 360
                            }
                        }

                    // Center content
                    VStack(spacing: 8) {
                        Text("\(Int(progress * 100))%")
                            .font(.system(size: 40, weight: .bold))
                            .foregroundColor(.white)

                        Text("Complete")
                            .font(.system(size: 16, weight: .medium))
                            .foregroundColor(.white.opacity(0.7))
                    }
                }
                .padding(40)
                .background(
                    Circle()
                        .fill(Color.white.opacity(0.05))
                        .blur(radius: 2)
                )
                .shadow(color: Color.blue.opacity(0.2), radius: 20, x: 0, y: 10)

                // Navigation to DoneView
                NavigationLink(destination: DoneView(), isActive: $showingDoneView) {
                    EmptyView()
                }
            }
            .padding()
        }
        .onAppear {
            simulateProgress()
        }
    }

    func simulateProgress() {
        Timer.scheduledTimer(withTimeInterval: 0.1, repeats: true) { timer in
            withAnimation(.easeInOut(duration: 0.2)) {
                if progress < 1.0 {
                    progress += 0.01
                } else {
                    timer.invalidate()
                    // Show the done view when progress completes
                    showingDoneView = true
                }
            }
        }
    }
}
