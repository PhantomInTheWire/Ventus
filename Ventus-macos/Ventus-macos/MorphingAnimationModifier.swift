//
//  MorphingAnimationModifier.swift
//  Ventus-macos
//
//  Created by Karan Haresh Lokchandani on 19/11/24.
//
import SwiftUI

struct MorphingAnimationModifier: ViewModifier {
    let phase: CGFloat
    
    func body(content: Content) -> some View {
        content
            .modifier(
                CustomGeometryEffect(phase: phase)
            )
    }
}
