//
//  AnimatedPhraseView.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 18/11/24.
//


import SwiftUI

struct AnimatedPhraseView: View {
    let phrases = [
        "Connecting to server... 🔄",
        "Checking timestamps... ⏱️",
        "Comparing files... 📋",
        "Establishing secure connection... 🔒",
        "Analyzing changes... 🔍",
        "Preparing files... 📁",
        "Calculating differences... ⚡️",
        "Verifying integrity... ✅",
        "Optimizing sync... ⚙️",
        "Updating metadata... 📝"
    ]

    @State private var currentPhraseIndex = 0
    @State private var opacity = 1.0

    var body: some View {
        Text(phrases[currentPhraseIndex])
            .font(.system(size: 15, weight: .medium)) // Adjusted size for macOS
            .foregroundStyle(
                LinearGradient(
                    colors: [.blue.opacity(0.8), .purple.opacity(0.8)],
                    startPoint: .leading,
                    endPoint: .trailing
                )
            )
            .opacity(opacity)
            .shadow(color: .blue.opacity(0.3), radius: 5)
            .multilineTextAlignment(.center)
            .onAppear {
                startPhraseAnimation()
            }
    }

    private func startPhraseAnimation() {
        Timer.scheduledTimer(withTimeInterval: 1.5, repeats: true) { _ in
            withAnimation(.easeOut(duration: 0.5)) {
                opacity = 0
            }

            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
                currentPhraseIndex = (currentPhraseIndex + 1) % phrases.count
                withAnimation(.easeIn(duration: 0.5)) {
                    opacity = 1
                }
            }
        }
    }
}
