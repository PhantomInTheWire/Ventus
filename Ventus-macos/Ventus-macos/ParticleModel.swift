//
//  ParticleModel.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

// MARK: - Models
struct ParticleModel: Identifiable {
    let id = UUID()
    var offset: CGSize
    var scale: CGFloat
    var angle: Double
    var opacity: Double
    
    static func random() -> ParticleModel {
        ParticleModel(
            offset: CGSize(
                width: .random(in: -150...250),
                height: .random(in: -150...250)
            ),
            scale: .random(in: 0.2...0.7),
            angle: .random(in: 0...360),
            opacity: .random(in: 0.3...0.7)
        )
    }
}


// MARK: - Style Definitions
struct SyncViewStyle {
    static let gradientColors = [
        Color(#colorLiteral(red: 0.1568627451, green: 0.1568627451, blue: 0.4, alpha: 1)),
        Color(#colorLiteral(red: 0.07843137255, green: 0.07843137255, blue: 0.2392156863, alpha: 1))
    ]
    
    struct Text {
        static let titleSize: CGFloat = 26
        static let percentageSize: CGFloat = 32
        static let percentageSymbolSize: CGFloat = 20
    }
    
    struct Circle {
        static let progressLineWidth: CGFloat = 12
        static let spinnerLineWidth: CGFloat = 2
    }
}

// MARK: - Subviews
struct BackgroundView: View {
    let colors: [Color]
    
    var body: some View {
        LinearGradient(colors: colors, startPoint: .top, endPoint: .bottom)
            .overlay(NoiseOverlay())
            .ignoresSafeArea()
    }
}

struct NoiseOverlay: View {
    var body: some View {
        Canvas { context, size in
            for _ in 0..<100 {
                let x = CGFloat.random(in: 0...size.width)
                let y = CGFloat.random(in: 0...size.height)
                context.opacity = Double.random(in: 0.01...0.05)
                context.fill(
                    Path(ellipseIn: CGRect(x: x, y: y, width: 2, height: 2)),
                    with: .color(.white)
                )
            }
        }
    }
}
