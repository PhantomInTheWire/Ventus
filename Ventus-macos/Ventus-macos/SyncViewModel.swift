//
//  SyncViewModel.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//
import SwiftUI

// MARK: - View Models
class SyncViewModel: ObservableObject {
    @Published var progress: Float = 0.0
    @Published var rotation: Double = 0.0
    @Published var showingDoneView = false
    @Published var morphingPhase: CGFloat = 0
    @Published var particles: [ParticleModel]
    
    private var progressTimer: Timer?
    
    init(particleCount: Int = 40) {
        self.particles = (0..<particleCount).map { _ in ParticleModel.random() }
    }
    
    func startAnimations() {
        startRotationAnimation()
        startMorphingAnimation()
        startProgressSimulation()
    }
    
    private func startRotationAnimation() {
        withAnimation(.linear(duration: 2).repeatForever(autoreverses: false)) {
            rotation = 360
        }
    }
    
    private func startMorphingAnimation() {
        withAnimation(.easeInOut(duration: 3).repeatForever(autoreverses: true)) {
            morphingPhase = 1
        }
    }
    
    private func startProgressSimulation() {
        progressTimer = Timer.scheduledTimer(withTimeInterval: 0.05, repeats: true) { [weak self] timer in
            guard let self = self else { return }
            withAnimation(.easeInOut(duration: 0.2)) {
                if self.progress < 1.0 {
                    self.progress += 0.005
                } else {
                    timer.invalidate()
                    self.progressComplete()
                }
            }
        }
    }
    
    private func progressComplete() {
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) {
            withAnimation {
                self.showingDoneView = true
            }
        }
    }
    var isLoadingComplete: Bool {
        return progress >= 1.0
    }
    deinit {
        progressTimer?.invalidate()
    }
}
