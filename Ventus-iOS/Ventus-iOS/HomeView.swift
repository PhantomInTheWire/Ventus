//
//  HomeView.swift
//  Ventus-iOS
//
//  Created by Karan Haresh Lokchandani on 15/11/24.
//

import SwiftUI

struct HomeView: View {
    @State private var ipAddress: String = ""
    @State private var showingSyncView = false
    
    private let gradientColors = [
        Color(#colorLiteral(red: 0.2, green: 0.2, blue: 0.5, alpha: 1)), // Deep purple
        Color(#colorLiteral(red: 0.1, green: 0.1, blue: 0.3, alpha: 1))  // Darker purple
    ]
    
    var body: some View {
        NavigationView {
            ZStack {
                AnimatedBackgroundView(gradientColors: gradientColors)
                // Background with radial gradient
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
                
                VStack(spacing: 30) {
                    Spacer()
                    
                    // App icon
                    Image(systemName: "network")
                        .resizable()
                        .scaledToFit()
                        .frame(width: 100, height: 100)
                        .foregroundStyle(
                            LinearGradient(
                                gradient: Gradient(colors: [Color.blue, Color.purple]),
                                startPoint: .top,
                                endPoint: .bottom
                            )
                        )
                        .shadow(color: .black.opacity(0.2), radius: 10, x: 0, y: 5)
                    
                    // Title with gradient
                    Text("Ventus")
                        .font(.system(size: 36, weight: .bold))
                        .foregroundStyle(
                            LinearGradient(
                                gradient: Gradient(colors: [Color.blue, Color.purple, Color.teal]),
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                        .shadow(color: .black.opacity(0.2), radius: 10, x: 0, y: 5)
                    
                    Text("Asynchronous FTP Sync")
                        .font(.system(size: 18, weight: .medium))
                        .foregroundColor(.white.opacity(0.8))
                        .padding(.bottom, 10)
                    
                    
                    // IP input field
                    HStack {
                        Image(systemName: "ip")
                            .foregroundColor(.white.opacity(0.8))
                        
                        TextField("Enter IP Address", text: $ipAddress)
                            .foregroundColor(.white.opacity(0.9))
                            .padding()
                            .keyboardType(.numbersAndPunctuation)
                    }
                    .padding()
                    .background(
                        RoundedRectangle(cornerRadius: 15)
                            .fill(Color.white.opacity(0.2))
                            .blur(radius: 2)
                            .shadow(color: .black.opacity(0.1), radius: 10, x: -5, y: 5)
                            .shadow(color: .white.opacity(0.2), radius: 10, x: 5, y: -5)
                    )
                    .padding(.horizontal)
                    
                    // Connect button
                    NavigationLink(destination: SyncView(), isActive: $showingSyncView) {
                        Button(action: {
                            if isValidIP(ipAddress) {
                                showingSyncView = true
                            }
                        }) {
                            HStack {
                                Image(systemName: "link")
                                    .font(.system(size: 20, weight: .bold))
                                Text("Connect")
                                    .font(.system(size: 20, weight: .bold))
                            }
                            .padding()
                            .frame(maxWidth: .infinity)
                            .background(
                                LinearGradient(
                                    gradient: Gradient(colors: [Color.blue, Color.purple]),
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                                .opacity(isValidIP(ipAddress) ? 1 : 0.5)
                            )
                            .foregroundColor(.white)
                            .cornerRadius(20)
                            .shadow(color: Color.blue.opacity(0.5), radius: 10, x: 0, y: 5)
                        }
                    }
                    .disabled(!isValidIP(ipAddress))
                    .padding(.horizontal)
                    .padding(.top, 20)
                    
                    Spacer()
                }
                .padding(.top, 40)
            }
            .navigationBarHidden(true)
        }
    }
    
    func isValidIP(_ ip: String) -> Bool {
        let parts = ip.split(separator: ".")
        guard parts.count == 4 else { return false }
        
        return parts.allSatisfy { part in
            guard let number = Int(part) else { return false }
            return number >= 0 && number <= 255
        }
    }
}
