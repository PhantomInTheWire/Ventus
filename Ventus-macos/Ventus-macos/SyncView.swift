import SwiftUI

struct SyncView: View {
    @StateObject private var viewModel = SyncViewModel()
    @State private var navigateToDoneView = false  // State to control navigation
    
    var body: some View {
        NavigationStack {  // Wrapping the view in a NavigationStack for navigation support
            ZStack {
                GeometryReader { geometry in
                    ZStack {
                    BackgroundView(colors: SyncViewStyle.gradientColors)
                        
                        ForEach(viewModel.particles) { particle in
                            ParticleView(particle: particle)
                        }
                        
                        VStack {
                            titleView
                            Spacer()
                            ProgressCircleView(
                                viewModel: viewModel,
                                size: geometry.size.width
                            )
                            .frame(height: geometry.size.height * 0.4)
                            Spacer()
                            AnimatedPhraseView()
                                .frame(height: 40)
                                .padding(.horizontal)
                        }
                        .navigationBarBackButtonHidden(true)
                        .padding(.horizontal)
                    }
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)
                .onAppear {
                    viewModel.startAnimations()
                }
                .onChange(of: viewModel.isLoadingComplete) { isComplete in
                    if isComplete {
                        navigateToDoneView = true
                    }
                }
            }
            .onChange(of: viewModel.isLoadingComplete) {
                isComplete in
                if isComplete {
                    navigateToDoneView = true
                }
            }

            // Use navigationDestination to handle the transition:
            .navigationDestination(isPresented: $navigateToDoneView) {
                DoneView()
            }
        }
    }
    
    private var titleView: some View {
        Text("Let the Magic Begin...")
            .font(.system(
                size: SyncViewStyle.Text.titleSize,
                weight: .bold
            ))
            .foregroundStyle(
                LinearGradient(
                    colors: [.blue.opacity(0.8), .purple.opacity(0.8)],
                    startPoint: .leading,
                    endPoint: .trailing
                )
            )
            .shadow(color: .blue.opacity(0.3), radius: 10)
            .overlay(titleShine)
            .padding(.top, 20)
    }
    
    private var titleShine: some View {
        Rectangle()
            .fill(
                LinearGradient(
                    colors: [.clear, .white.opacity(0.2), .clear],
                    startPoint: .leading,
                    endPoint: .trailing
                )
            )
            .offset(x: -200)
            .rotationEffect(.degrees(30))
            .mask(
                Text("Let the Magic Begin...")
                    .font(.system(
                        size: SyncViewStyle.Text.titleSize,
                        weight: .bold
                    ))
            )
            .animation(
                .linear(duration: 2)
                .repeatForever(autoreverses: false),
                value: viewModel.rotation
            )
    }
}

#Preview {
    SyncView()
}
