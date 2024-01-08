// Note: This is a reverse-engineered header for omni::physx plugin until the real one is released

#pragma once

#include <carb/Interface.h>
#include <carb/Types.h>

#include <typeinfo>

namespace omni
{
    namespace physx
    {
        using OnStageResultFn = std::function<void(bool result, const char *err)>;

        struct IPhysx
        {
            CARB_PLUGIN_INTERFACE("omni::physx::IPhysx", 2, 0);

            // Kinda sure
            void(CARB_ABI *updateSimulationScene)(unsigned long scenePath, float elapsedStep, float currentTime);
            void(CARB_ABI *updateTransformationsScene)(unsigned long scenePath, bool updateToUsd, bool updateVelocitiesToUsd);

            // Maybe
            void(CARB_ABI *forceLoadPhysicsFromUsd)();
            void(CARB_ABI *startSimulation)();

            // Not yet sure
            void(CARB_ABI *updateSimulation)(float elapsedStep, float currentTime);
            void(CARB_ABI *updateTransformations)(bool updateToUsd, bool updateVelocitiesToUsd);
            // virtual void updateSimulation(float elapsedStep, float currentTime) = 0;
            // virtual void updateTransformations(bool updateToUsd, bool updateVelocitiesToUsd) = 0;
        };

        struct IPhysxSimulation
        {
            CARB_PLUGIN_INTERFACE("omni::physx::IPhysxSimulation", 1, 0);

            bool(CARB_ABI *attachStage)(long stageId);
            bool(CARB_ABI *checkResultsScene)(unsigned long scenePath);
            bool(CARB_ABI *checkResults)();
            void(CARB_ABI *pauseChangeTracking)(bool pause);
            void(CARB_ABI *simulate)(float elapsedTime, float currentTime);
            void(CARB_ABI *simulateScene)(unsigned long scenePath, float elapsedTime, float currentTime);
            void(CARB_ABI *fetchResultsScene)(unsigned long scenePath);
            void(CARB_ABI *detachStage)();

            // Might be a different definition
            void(CARB_ABI *fetchResults)();
            // virtual void fetchResults() = 0;
        };
    } // namespace physx
} // namespace omni
