//
//  CustomGeometryEffect.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//

import SwiftUI

struct CustomGeometryEffect: GeometryEffect {
    var phase: CGFloat
    
    var animatableData: CGFloat {
        get { phase }
        set { phase = newValue }
    }
    
    func effectValue(size: CGSize) -> ProjectionTransform {
        let angle = 2 * .pi * phase
        let translation = CGFloat(4 * sin(angle))
        let scaleFactor = 1.0 + 0.1 * sin(angle * 2)
        
        var transform = CGAffineTransform.identity
        transform = transform.translatedBy(x: translation, y: 0)
        transform = transform.scaledBy(x: scaleFactor, y: scaleFactor)
        
        return ProjectionTransform(transform)
    }
}
